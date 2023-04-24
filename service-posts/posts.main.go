package main

import (
	"context"
	"database/sql"
	"fmt"
	"log"
	"net"

	"github.com/go-playground/validator/v10"
	"github.com/golang-jwt/jwt"
	_ "github.com/jackc/pgx/v5/stdlib"
	migrate "github.com/rubenv/sql-migrate"
	"google.golang.org/grpc"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/metadata"
	"google.golang.org/grpc/status"

	pb "rusve/proto"

	utils "github.com/mpiorowski/golang"
)

var db *sql.DB

type server struct {
	pb.UnsafePostsServiceServer
}

var (
	PORT         = utils.MustGetenv("PORT")
	ENV          = utils.MustGetenv("ENV")
	DATABASE_URL = utils.MustGetenv("DATABASE_URL")
	SECRET       = utils.MustGetenv("SECRET")
)

var validate = validator.New()

func init() {
	// Db connection
	var err error

	if db, err = sql.Open("pgx", DATABASE_URL); err != nil {
		log.Fatal(err)
	}
	pingErr := db.Ping()
	if pingErr != nil {
		log.Fatal(pingErr)
	}
	log.Println("Connected to database")

	// Migrations
	var migrationsDir = "./migrations"
	if ENV == "production" {
		migrationsDir = "/migrations"
	}
	migrations := &migrate.FileMigrationSource{
		Dir: migrationsDir,
	}
	n, err := migrate.Exec(db, "postgres", migrations, migrate.Up)
	if err != nil {
		log.Fatalf("Migrations failed: %v", err)
	}
	log.Printf("Applied migrations: %d", n)
}

func main() {

	lis, err := net.Listen("tcp", fmt.Sprintf(":%v", PORT))
	if err != nil {
		log.Fatalf("Failed to listen: %v", err)
	}
	// this is not catching the requests
	s := grpc.NewServer(
		grpc.StreamInterceptor(streamInterceptor),
		grpc.UnaryInterceptor(unaryInterceptor),
	)
	pb.RegisterPostsServiceServer(s, &server{})
	log.Printf("Server listening at: %v", lis.Addr())
	if err := s.Serve(lis); err != nil {
		log.Fatalf("Failed to serve: %v", err)
	}
}

type Claims struct {
	UserId string `json:"user_id"`
	jwt.StandardClaims
}

type User struct {
	Id string `json:"id"`
}

func decodeToken(md metadata.MD) (string, error) {
	authHeaders, ok := md["authorization"]
	if !ok {
		return "", status.Errorf(codes.Unauthenticated, "Authorization header is not provided")
	}
	authHeader := authHeaders[0]

	// Check if the authorization header is valid
	if len(authHeader) < 7 || authHeader[:7] != "Bearer " {
		return "", status.Errorf(codes.Unauthenticated, "Authorization header is not valid")
	}
	authHeader = authHeader[7:]

	// Parse the JWT token
	token, err := jwt.ParseWithClaims(authHeader, &Claims{}, func(token *jwt.Token) (interface{}, error) {
		return []byte(SECRET), nil
	})
	if err != nil || !token.Valid {
		return "", status.Errorf(codes.Unauthenticated, "Invalid authorization token")
	}

	return token.Claims.(*Claims).UserId, nil
}

func streamInterceptor(
	srv interface{},
	ss grpc.ServerStream,
	info *grpc.StreamServerInfo,
	handler grpc.StreamHandler,
) error {
	// Extract the JWT token from the metadata
	md, ok := metadata.FromIncomingContext(ss.Context())
	if !ok {
		return status.Errorf(codes.Unauthenticated, "Metadata is not provided")
	}
	_, err := decodeToken(md)
	if err != nil {
		return status.Errorf(codes.Unauthenticated, "Invalid authorization token")
	}
	// Add the token to the context for downstream handlers to use

    // TODO - find a way to pass the context to the handler
    // ctx := context.WithValue(ss.Context(), User{}, userId)

    return handler(srv, ss)
}

func unaryInterceptor(
	ctx context.Context,
	req interface{},
	info *grpc.UnaryServerInfo,
	handler grpc.UnaryHandler,
) (interface{}, error) {
	// Extract the JWT token from the metadata
	md, ok := metadata.FromIncomingContext(ctx)
	if !ok {
		return nil, status.Errorf(codes.Unauthenticated, "Metadata is not provided")
	}
    userId, err := decodeToken(md)
    if err != nil {
        return nil, status.Errorf(codes.Unauthenticated, "Invalid authorization token")
    }
	// Add the token to the context for downstream handlers to use
	ctx = context.WithValue(ctx, User{}, userId)

	return handler(ctx, req)
}
