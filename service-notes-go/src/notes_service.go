package main

import (
	"context"
	"log"
	"time"

	pb "rusve/proto"

	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (s *server) GetNotes(in *pb.UserId, stream pb.NotesService_GetNotesServer) error {
	log.Printf("GetNotes: %v", in.UserId)

	start := time.Now()

	rows, err := db.Query(`select * from notes where deleted is null order by created desc`)
	if err != nil {
		log.Printf("db.Query: %v", err)
		return err
	}
	defer rows.Close()

	for rows.Next() {
		note, err := mapNote(rows, nil)
		if err != nil {
			log.Printf("mapNote: %v", err)
			return err
		}
		err = stream.Send(note)
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

func (s *server) CreateNote(ctx context.Context, in *pb.Note) (*pb.Empty, error) {
	log.Printf("CreateNote: %v", in)

	rules := map[string]string{
		"UserId":  "required,max=100",
		"Title":   "required,max=100",
		"Content": "required,max=1000",
	}
	validate.RegisterStructValidationMapRules(rules, pb.Note{})
	err := validate.Struct(in)
	if err != nil {
		log.Printf("validate.Struct: %v", err)
		return nil, status.Error(codes.InvalidArgument, "Invalid argument")
	}

	if len(in.Id) == 0 {
		// for benchmarks, delete all notes and create 5000 new ones
		_, err = db.Exec(`delete from notes where user_id = $1`, in.UserId)
		if err != nil {
			log.Printf("db.Exec: %v", err)
			return nil, err
		}
		for i := 0; i < 5000; i++ {
			_, err = db.Exec(`insert into notes (user_id, title, content) values ($1, $2, $3)`, in.UserId, in.Title, in.Content)
			if err != nil {
				log.Printf("db.Exec: %v", err)
				return nil, err
			}
		}
	} else {
		_, err = db.Exec(`update notes set title = $1, content = $2 where id = $3 and user_id = $4`, in.Title, in.Content, in.Id, in.UserId)
		if err != nil {
			log.Printf("db.Exec: %v", err)
			return nil, err
		}
	}
	if err != nil {
		log.Printf("mapNote: %v", err)
		return nil, err
	}
	return &pb.Empty{}, nil
}

func (s *server) DeleteNote(ctx context.Context, in *pb.NoteId) (*pb.Empty, error) {
	log.Printf("DeleteNote: %v", in)

	_, err := db.Exec(`update notes set deleted = now() where id = $1 and user_id = $2`, in.NoteId, in.UserId)
	if err != nil {
		log.Printf("db.Exec: %v", err)
		return nil, err
	}

	return &pb.Empty{}, nil
}
