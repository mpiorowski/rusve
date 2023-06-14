package main

import (
	"context"
	"database/sql"
	"log"

	pb "rusve/proto"

	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

/**
* Check if user exists, if not create new user
 */
func (s *server) Auth(ctx context.Context, in *pb.AuthRequest) (*pb.User, error) {
	log.Println("Auth")

	rules := map[string]string{
		"Email": "required,max=100,email",
		"Sub":   "required,max=100",
	}
	validate.RegisterStructValidationMapRules(rules, pb.AuthRequest{})
	err := validate.Struct(in)
	if err != nil {
		log.Printf("validate.Struct: %v", err)
		return nil, status.Error(codes.InvalidArgument, "Invalid email or code")
	}

	row := db.QueryRow(`select * from users where email = $1`, in.Email)
	user, err := mapUser(nil, row)
	if err != nil && err != sql.ErrNoRows {
		log.Printf("mapUser: %v", err)
		return nil, err
	}

	if user.GetDeleted() != "" {
		return nil, status.Error(codes.Unauthenticated, "Unauthenticated")
	}

	if err == sql.ErrNoRows {
		role := pb.UserRole_ROLE_USER
		println("role", role)
		row = db.QueryRow(`insert into users (email, role, sub) values ($1, $2, $3) returning *`, in.Email, role, in.Sub)
		user, err = mapUser(nil, row)
		if err != nil {
			log.Printf("mapUser: %v", err)
			return nil, err
		}
	}

	return user, nil
}

func (s *server) GetUsers(in *pb.UserIds, stream pb.UsersService_GetUsersServer) error {
	log.Printf("GetUsers")

	rows, err := db.Query(`select * from users where id = any($1)`, in.UserIds)
	if err != nil {
		log.Printf("db.Query: %v", err)
		return err
	}
	defer rows.Close()

	for rows.Next() {
		user, err := mapUser(rows, nil)
		if err != nil {
			log.Printf("mapUser: %v", err)
			return err
		}
		err = stream.Send(user)
		if err != nil {
			log.Printf("stream.Send: %v", err)
			return err
		}
	}
	if rows.Err() != nil {
		log.Printf("rows.Err: %v", err)
		return err
	}
	return nil
}

func (s *server) GetUser(ctx context.Context, in *pb.UserId) (*pb.User, error) {
	log.Printf("GetUser")

	row := db.QueryRow(`select * from users where id = $1`, in.UserId)
	user, err := mapUser(nil, row)
	if err != nil {
		log.Printf("mapUser: %v", err)
		return nil, err
	}
	return user, nil
}

func (s *server) UpdateUser(ctx context.Context, in *pb.User) (*pb.User, error) {
	log.Printf("UpdateUser")

	row := db.QueryRow(`update users set name = $1, avatar_id = $2 where id = $3 and deleted is null returning *`, in.Name, in.AvatarId, in.Id)
	user, err := mapUser(nil, row)
	if err != nil {
		log.Printf("mapUser: %v", err)
		return nil, err
	}
	return user, nil
}

func (s *server) DeleteUser(ctx context.Context, in *pb.User) (*pb.User, error) {
	log.Printf("DeleteUser")

	row := db.QueryRow(`update users set deleted = now() where id = $1 and sub = $2 and email = $3 returning *`, in.Id, in.Sub, in.Email)
	user, err := mapUser(nil, row)
	if err != nil {
		log.Printf("db.Exec: %v", err)
		return nil, err
	}
	return user, nil
}
