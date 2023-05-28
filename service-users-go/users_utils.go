package main

import (
	"database/sql"
	pb "rusve/proto"
)

func mapUser(rows *sql.Rows, row *sql.Row) (*pb.User, error) {
	var user pb.User
	var err error
	var role string
	if rows != nil {
		err = rows.Scan(
			&user.Id, &user.Created, &user.Updated, &user.Deleted, &user.Email, &role, &user.Sub, &user.Name, &user.AvatarId, &user.PaymentId,
		)
	} else if row != nil {
		err = row.Scan(
			&user.Id, &user.Created, &user.Updated, &user.Deleted, &user.Email, &role, &user.Sub, &user.Name, &user.AvatarId, &user.PaymentId,
		)
	}
	if err != nil {
		return nil, err
	}
    user.Role = pb.UserRole(pb.UserRole_value[role])
	return &user, nil
}
