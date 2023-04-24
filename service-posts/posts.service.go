package main

import (
	"context"
	"database/sql"
	"log"
	"time"

	pb "rusve/proto"

	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (s *server) GetPosts(in *pb.Empty, stream pb.PostsService_GetPostsServer) error {
	log.Println("GetPosts")
	start := time.Now()

	rows, err := db.Query(`select * from posts where deleted is null order by created desc`)
	if err != nil {
		log.Printf("db.Query: %v", err)
		return err
	}
	defer rows.Close()

	for rows.Next() {
		post, err := mapPost(rows, nil)
		if err != nil {
			log.Printf("mapPost: %v", err)
			return err
		}
		err = stream.Send(post)
		if err != nil {
			log.Printf("stream.Send: %v", err)
			return err
		}
	}
	end := time.Now()
    log.Printf("Elapsed: %v", end.Sub(start))
	if rows.Err() != nil {
		log.Printf("rows.Err: %v", err)
		return err
	}
	return nil
}

func (s *server) CreatePost(ctx context.Context, in *pb.Post) (*pb.Post, error) {
	log.Println("CreatePost")

	rules := map[string]string{
		"UserId":  "required,max=100",
		"Title":   "required,max=100",
		"Content": "required,max=1000",
	}
	validate.RegisterStructValidationMapRules(rules, pb.Post{})
	err := validate.Struct(in)
	if err != nil {
		log.Printf("validate.Struct: %v", err)
		return nil, status.Error(codes.InvalidArgument, "Invalid argument")
	}

	var row *sql.Row
	if in.Id == "" {
		row = db.QueryRow(`insert into posts (user_id, title, content) values ($1, $2, $3) returning *`, in.UserId, in.Title, in.Content)
	} else {
		row = db.QueryRow(`update posts set title = $1, content = $2 where id = $3 and user_id = $4 returning *`, in.Title, in.Content, in.Id, in.UserId)
	}
	post, err := mapPost(nil, row)
	if err != nil {
		log.Printf("mapPost: %v", err)
		return nil, err
	}
	return post, nil
}

func (s *server) DeletePost(ctx context.Context, in *pb.PostId) (*pb.Post, error) {
	log.Println("DeletePost")

	row := db.QueryRow(`update posts set deleted = now() where id = $1 and user_id = $2 returning *`, in.PostId, in.UserId)
	post, err := mapPost(nil, row)
	if err != nil {
		log.Printf("mapPost: %v", err)
		return nil, err
	}
	return post, nil
}
