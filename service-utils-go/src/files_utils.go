package main

import (
	"context"
	"database/sql"
	"io"
	"os"
	"time"

	pb "rusve/proto"

	"cloud.google.com/go/storage"
)

func mapFile(rows *sql.Rows, row *sql.Row) (*pb.File, error) {
	var file pb.File = pb.File{}
	var err error
	var _type string
	if rows != nil {
		err = rows.Scan(
			&file.Id,
			&file.Created,
			&file.Updated,
			&file.Deleted,
			&file.TargetId,
			&file.Name,
			&_type,
		)
	} else if row != nil {
		err = row.Scan(
			&file.Id,
			&file.Created,
			&file.Updated,
			&file.Deleted,
			&file.TargetId,
			&file.Name,
			&_type,
		)
	}
	if err != nil {
		return nil, err
	}
	file.Type = pb.FileType(pb.FileType_value[_type])
	return &file, nil
}

func uploadFile(fileId string, name string, data []byte) error {
	if ENV == "development" {
		// save to local disk, inside /files folder
		err := os.MkdirAll("/app/files/"+fileId, 0755)
		if err != nil {
			return err
		}
		err = os.WriteFile("/app/files/"+fileId+"/"+name, data, 0644)
		if err != nil {
			return err
		}
		return nil
	}
	ctx := context.Background()
	client, err := storage.NewClient(ctx)
	if err != nil {
		return err
	}
	defer client.Close()

	ctx, cancel := context.WithTimeout(ctx, time.Second*50)
	defer cancel()

	o := client.Bucket(BUCKET).Object(fileId + "/" + name)
	wc := o.NewWriter(ctx)
	_, err = wc.Write(data)
	if err != nil {
		return err
	}
	err = wc.Close()
	if err != nil {
		return err
	}
	return nil
}

func downloadFile(fileId string, name string) ([]byte, error) {
	if ENV == "development" {
		// download from local disk, inside /files folder
		data, err := os.ReadFile("/app/files/" + fileId + "/" + name)
		if err != nil {
			return nil, err
		}
		return data, nil
	}
	ctx := context.Background()
	client, err := storage.NewClient(ctx)
	if err != nil {
		return nil, err
	}
	defer client.Close()

	ctx, cancel := context.WithTimeout(ctx, time.Second*50)
	defer cancel()

	rc, err := client.Bucket(BUCKET).Object(fileId + "/" + name).NewReader(ctx)
	if err != nil {
		return nil, err
	}
	defer rc.Close()

	buffer, err := io.ReadAll(rc)
	if err != nil {
		return nil, err
	}
	return buffer, nil
}

func deleteFile(fileId string, name string) error {
	if ENV == "development" {
		err := os.Remove("/app/files/" + fileId + "/" + name)
		if err != nil {
			return err
		}
		err = os.Remove("/app/files/" + fileId)
		if err != nil {
			return err
		}
		return nil
	}
	ctx := context.Background()
	client, err := storage.NewClient(ctx)
	if err != nil {
		return err
	}
	defer client.Close()

	ctx, cancel := context.WithTimeout(ctx, time.Second*50)
	defer cancel()

	err = client.Bucket(BUCKET).Object(fileId + "/" + name).Delete(ctx)
	if err != nil {
		return err
	}
	return nil
}

func generateV4GetObjectSignedURL(fileId string, fileName string) (string, error) {
	object := fileId + "/" + fileName
	ctx := context.Background()
	client, err := storage.NewClient(ctx)
	if err != nil {
		return "", err
	}
	defer client.Close()

	opts := &storage.SignedURLOptions{
		Scheme:  storage.SigningSchemeV4,
		Method:  "GET",
		Expires: time.Now().Add(15 * time.Minute),
	}

	u, err := client.Bucket(BUCKET).SignedURL(object, opts)
	if err != nil {
		return "", err
	}
	return u, nil
}
