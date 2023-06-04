package main

import (
	"context"
	"log"

	pb "rusve/proto"
)

func (s *server) GetFiles(in *pb.TargetId, stream pb.UsersService_GetFilesServer) error {
	log.Printf("GetFiles: %v", in)

	rules := map[string]string{
		"TargetId": "required,max=100,uuid",
		"Type":     "required,max=100",
	}
	validate.RegisterStructValidationMapRules(rules, pb.TargetId{})
	err := validate.Struct(in)
	if err != nil {
		log.Printf("validate.Struct: %v", err)
		return err
	}

	rows, err := db.Query(`select * from files where target_id = $1 and type = $2 and deleted is null`, in.TargetId, in.Type)
	if err != nil {
		log.Printf("db.Query: %v", err)
		return err
	}
	defer rows.Close()

	for rows.Next() {
		file, err := mapFile(rows, nil)
		if err != nil {
			log.Printf("mapFile: %v", err)
			return err
		}

		if ENV == "production" {
			file.Url, err = generateV4GetObjectSignedURL(file.TargetId + "/" + file.Name)
			if err != nil {
				log.Printf("generateV4GetObjectSignedURL: %v", err)
				return err
			}
		} else {
			file.Url = ""
		}

		err = stream.Send(file)
		if err != nil {
			log.Printf("stream.Send: %v", err)
			return err
		}
	}
	if rows.Err() != nil {
		log.Printf("rows.Err: %v", err)
		return rows.Err()
	}
	return nil
}

func (s *server) GetFile(ctx context.Context, in *pb.FileId) (*pb.File, error) {
	log.Printf("GetFile: %v", in)

	rules := map[string]string{
		"FileId":   "required,max=100,uuid",
		"TargetId": "required,max=100,uuid",
	}
	validate.RegisterStructValidationMapRules(rules, pb.TargetId{})
	err := validate.Struct(in)
	if err != nil {
		log.Printf("validate.Struct: %v", err)
		return nil, err
	}

	row := db.QueryRow(`select * from files where id = $1 and target_id = $2 and deleted is null`, in.FileId, in.TargetId)
	file, err := mapFile(nil, row)
	if err != nil {
		log.Printf("mapFile: %v", err)
		return nil, err
	}

	buffer, err := downloadFile(file.TargetId, file.Name)
	if err != nil {
		log.Printf("downloadFile: %v", err)
		return nil, err
	}
	file.Buffer = buffer
	return file, nil
}

func (s *server) CreateFile(ctx context.Context, in *pb.File) (*pb.File, error) {
	log.Printf("CreateFile")

	rules := map[string]string{
		"TargetId": "required,max=100,uuid",
		"Name":     "required,max=100",
		"Type":     "required,max=100",
		"Buffer":   "required",
	}
	validate.RegisterStructValidationMapRules(rules, pb.File{})
	err := validate.Struct(in)
	if err != nil {
		log.Printf("validate.Struct: %v", err)
		return nil, err
	}

	err = uploadFile(in.TargetId, in.Name, in.Buffer)
	if err != nil {
		log.Printf("uploadFile: %v", err)
		return nil, err
	}

	row := db.QueryRow(`insert into files (target_id, name, type) values ($1, $2, $3) returning *`,
		in.TargetId,
		in.Name,
		in.Type,
	)

	file, err := mapFile(nil, row)
	if err != nil {
		log.Printf("mapFile: %v", err)
		return nil, err
	}

	return file, nil
}

// TODO - delete form bucket
func (s *server) DeleteFile(ctx context.Context, in *pb.FileId) (*pb.File, error) {
	log.Printf("DeleteFile: %v", in)

	row := db.QueryRow(`update files set deleted = now() where id = $1 and target_id = $2 returning *`, in.FileId, in.TargetId)

	file, err := mapFile(nil, row)
	if err != nil {
		log.Printf("mapFile: %v", err)
		return nil, err
	}

	return file, nil
}
