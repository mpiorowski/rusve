package main

import (
	"database/sql"
	pb "rusve/proto"
)

func mapUser(rows *sql.Rows, row *sql.Row) (*pb.User, error) {
	var user pb.User
	var err error
	if rows != nil {
		err = rows.Scan(
			&user.Id, &user.Created, &user.Updated, &user.Deleted, &user.Email, &user.Role, &user.Sub,
		)
	} else if row != nil {
		err = row.Scan(
			&user.Id, &user.Created, &user.Updated, &user.Deleted, &user.Email, &user.Role, &user.Sub,
		)
	}
	if err != nil {
		return nil, err
	}
	return &user, nil
}
