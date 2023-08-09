package main

import (
	"context"
	"database/sql"
	"log/slog"
	"time"

	pb "rusve/proto"

	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (s *server) GetNotes(in *pb.UserId, stream pb.NotesService_GetNotesServer) error {
	start := time.Now()

	rows, err := db.Query(`select * from notes where user_id = $1 and deleted is null`, in.UserId)
	if err != nil {
		slog.Error("db.Query", "error", err)
		return err
	}
	defer rows.Close()

	for rows.Next() {
		note, err := mapNote(rows, nil)
		if err != nil {
			slog.Error("mapNote", "error", err)
			return err
		}
		err = stream.Send(note)
		if err != nil {
			slog.Error("stream.Send", "error", err)
			return err
		}
	}
	if rows.Err() != nil {
		slog.Error("rows.Err", "error", err)
		return err
	}
	slog.Info("GetNotes", "time", time.Since(start))
	return nil
}

func (s *server) CreateNote(ctx context.Context, in *pb.Note) (*pb.Note, error) {
	start := time.Now()
	rules := map[string]string{
		"UserId":  "required,max=100",
		"Title":   "required,max=100",
		"Content": "required,max=1000",
	}
	validate.RegisterStructValidationMapRules(rules, pb.Note{})
	err := validate.Struct(in)
	if err != nil {
		slog.Error("validate.Struct", "error", err)
		return nil, status.Error(codes.InvalidArgument, "Invalid argument")
	}

	var row *sql.Row
	if in.Id == "" {
		row = db.QueryRow(`insert into notes (user_id, title, content) values ($1, $2, $3) returning *`, in.UserId, in.Title, in.Content)
	} else {
		row = db.QueryRow(`update notes set title = $1, content = $2 where id = $3 and user_id = $4 returning *`, in.Title, in.Content, in.Id, in.UserId)
	}
	note, err := mapNote(nil, row)
	if err != nil {
		slog.Error("mapNote", "error", err)
		return nil, err
	}

	slog.Info("CreateNote", "time", time.Since(start))
	return note, nil
}

func (s *server) DeleteNote(ctx context.Context, in *pb.NoteId) (*pb.Empty, error) {
	start := time.Now()

	_, err := db.Exec(`update notes set deleted = now() where id = $1 and user_id = $2`, in.NoteId, in.UserId)
	if err != nil {
		slog.Error("db.Exec", "error", err)
		return nil, err
	}

	slog.Info("DeleteNote", "time", time.Since(start))
	return &pb.Empty{}, nil
}
