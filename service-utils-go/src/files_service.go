package main

import (
	"context"
	"log/slog"
	"time"

	pb "rusve/proto"
)

func (s *server) GetFiles(in *pb.TargetId, stream pb.UtilsService_GetFilesServer) error {
	start := time.Now()

	rules := map[string]string{
		"TargetId": "required,max=100",
		"Type":     "required,max=100",
	}
	validate.RegisterStructValidationMapRules(rules, pb.TargetId{})
	err := validate.Struct(in)
	if err != nil {
		slog.Error("GetFiles", "validate.Struct", err)
		return err
	}

	rows, err := db.Query(`select * from files where target_id = $1 and type = $2 and deleted is null`, in.TargetId, in.Type)
	if err != nil {
		slog.Error("GetFiles", "db.Query", err)
		return err
	}
	defer rows.Close()

	for rows.Next() {
		file, err := mapFile(rows, nil)
		if err != nil {
			slog.Error("GetFiles", "mapFile", err)
			return err
		}

		// in go, change byte array to string
		if ENV == "production" {
			file.Url, err = generateV4GetObjectSignedURL(file.Id, file.Name)
			if err != nil {
				slog.Error("GetFiles", "generateV4GetObjectSignedURL", err)
				return err
			}
		} else {
			file.Url = ""
		}

		err = stream.Send(file)
		if err != nil {
			slog.Error("GetFiles", "stream.Send", err)
			return err
		}
	}
	if rows.Err() != nil {
		slog.Error("GetFiles", "rows.Err", err)
		return rows.Err()
	}
	slog.Info("GetFiles", "time", time.Since(start))
	return nil
}

func (s *server) GetFile(ctx context.Context, in *pb.FileId) (*pb.File, error) {
	start := time.Now()

	rules := map[string]string{
		"FileId":   "required,max=100",
		"TargetId": "required,max=100",
	}
	validate.RegisterStructValidationMapRules(rules, pb.TargetId{})
	err := validate.Struct(in)
	if err != nil {
		slog.Error("GetFile", "validate.Struct", err)
		return nil, err
	}

	row := db.QueryRow(`select * from files where id = $1 and target_id = $2 and deleted is null`, in.FileId, in.TargetId)
	file, err := mapFile(nil, row)
	if err != nil {
		slog.Error("GetFile", "mapFile", err)
		return nil, err
	}

	buffer, err := downloadFile(file.Id, file.Name)
	if err != nil {
		slog.Error("GetFile", "downloadFile", err)
		return nil, err
	}
	if buffer == nil {
		slog.Error("GetFile", "buffer", "nil")
		return nil, err
	}
	file.Buffer = buffer

	slog.Info("GetFile", "time", time.Since(start))
	return file, nil
}

func (s *server) CreateFile(ctx context.Context, in *pb.File) (*pb.File, error) {
	start := time.Now()

	rules := map[string]string{
		"TargetId": "required,max=100",
		"Name":     "required,max=100",
		"Type":     "required,max=100",
		"Buffer":   "required",
	}
	validate.RegisterStructValidationMapRules(rules, pb.File{})
	err := validate.Struct(in)
	if err != nil {
		slog.Error("CreateFile", "validate.Struct", err)
		return nil, err
	}

	// TODO - transaction?
	row := db.QueryRow(`insert into files (target_id, name, type) values ($1, $2, $3) returning *`,
		in.TargetId,
		in.Name,
		in.Type,
	)

	file, err := mapFile(nil, row)
	if err != nil {
		slog.Error("CreateFile", "mapFile", err)
		return nil, err
	}

	err = uploadFile(file.Id, file.Name, in.Buffer)
	if err != nil {
		_, _ = db.Exec(`update files set deleted = now() where id = $1`, file.Id)
		return nil, err
	}

	slog.Info("CreateFile", "time", time.Since(start))
	return file, nil
}

func (s *server) DeleteFile(ctx context.Context, in *pb.FileId) (*pb.File, error) {
    start := time.Now()

	row := db.QueryRow(`update files set deleted = now() where id = $1 and target_id = $2 returning *`, in.FileId, in.TargetId)

	file, err := mapFile(nil, row)
	if err != nil {
        slog.Error("DeleteFile", "mapFile", err)
		return nil, err
	}

    err = deleteFile(file.Id, file.Name)
    if err != nil {
        slog.Error("DeleteFile", "deleteFile", err)
        return nil, err
    }

    slog.Info("DeleteFile", "time", time.Since(start))
	return file, nil
}
