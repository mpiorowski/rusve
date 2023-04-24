package main

import (
	"database/sql"
	pb "rusve/proto"
)

func mapPost(rows *sql.Rows, row *sql.Row) (*pb.Post, error) {
	var post pb.Post
	var err error
	if rows != nil {
		err = rows.Scan(
			&post.Id, &post.Created, &post.Updated, &post.Deleted, &post.UserId, &post.Title, &post.Content,
		)
	} else if row != nil {
		err = row.Scan(
			&post.Id, &post.Created, &post.Updated, &post.Deleted, &post.UserId, &post.Title, &post.Content,
		)
	}
	if err != nil {
		return nil, err
	}
	return &post, nil
}
