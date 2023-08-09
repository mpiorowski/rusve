package main

import (
	"context"
	"database/sql"
	"fmt"
	"log"
	"log/slog"
	"net"
	"os"
	"strings"

	"github.com/go-playground/validator/v10"
	"github.com/golang-jwt/jwt/v5"
	_ "github.com/jackc/pgx/v5/stdlib"

	// migrate "github.com/rubenv/sql-migrate"
	"google.golang.org/grpc"
	"google.golang.org/grpc/metadata"

	pb "rusve/proto"
)

var db *sql.DB

type server struct {
	pb.UnimplementedNotesServiceServer
}

func MustGetenv(k string) string {
	v := os.Getenv(k)
	if v == "" {
		log.Fatalf("Error: %s environment variable not set.\n", k)
	}
	return v
}

var (
	PORT         = MustGetenv("PORT")
	ENV          = MustGetenv("ENV")
	DATABASE_URL = MustGetenv("DATABASE_URL")
)

var validate = validator.New()

func init() {
	var err error

	if db, err = sql.Open("pgx", DATABASE_URL); err != nil {
		log.Fatal(err)
	}
	pingErr := db.Ping()
	if pingErr != nil {
		log.Fatal(pingErr)
	}
	slog.Info("Connected to database")

	// Example of running migrations
	// var migrationsDir = "./migrations"
	// if ENV == "production" {
	// 	migrationsDir = "/migrations"
	// }
	// migrations := &migrate.FileMigrationSource{
	// 	Dir: migrationsDir,
	// }
	// n, err := migrate.Exec(db, "postgres", migrations, migrate.Up)
	// if err != nil {
	// 	log.Fatalf("Migrations failed: %v", err)
	// }
	// log.Printf("Applied migrations: %d", n)
}

func main() {
	lis, err := net.Listen("tcp", fmt.Sprintf(":%v", PORT))
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}
	if err != nil {
		log.Fatalf("Failed to generate credentials: %v", err)
	}
	s := grpc.NewServer(grpc.UnaryInterceptor(check_auth))
	pb.RegisterNotesServiceServer(s, &server{})
	slog.Info("Server listening at", "address", lis.Addr())
	if err := s.Serve(lis); err != nil {
		log.Fatalf("Failed to serve: %v", err)
	}
}

func check_auth(
	ctx context.Context,
	req interface{},
	info *grpc.UnaryServerInfo,
	handler grpc.UnaryHandler,
) (interface{}, error) {
	// Extract the incoming metadata
	md, ok := metadata.FromIncomingContext(ctx)
	if !ok {
		return nil, fmt.Errorf("Missing metadata")
	}

	// Access the headers from the metadata
	token := md.Get("x-authorization")
	if len(token) == 0 {
		return nil, fmt.Errorf("Missing authorization header")
	}

	// Validate the token
	tokenParts := strings.SplitN(token[0], " ", 2)
	if len(tokenParts) != 2 || strings.ToLower(tokenParts[0]) != "bearer" {
		return nil, fmt.Errorf("Invalid authorization header")
	}

	// Get the public public key from file
	publicKey, err := os.ReadFile("./public.key")
	if err != nil {
		return nil, fmt.Errorf("Invalid public key")
	}

	// Decode the token
	tokenString := tokenParts[1]
	_, err = decodeToken(tokenString, publicKey)
	if err != nil {
		return nil, fmt.Errorf("Invalid token: %v", err)
	}

	// Call the gRPC handler to process the request
	return handler(ctx, req)
}

type Claims struct {
	sub string
}

func decodeToken(tokenString string, publicKey []byte) (*Claims, error) {
	token, err := jwt.Parse(tokenString, func(token *jwt.Token) (interface{}, error) {
		if _, ok := token.Method.(*jwt.SigningMethodRSA); !ok {
			return nil, fmt.Errorf("Unexpected signing method")
		}
		return jwt.ParseRSAPublicKeyFromPEM(publicKey)
	})
	if err != nil {
		return nil, err
	}

	claims, ok := token.Claims.(jwt.MapClaims)
	if !ok || !token.Valid {
		return nil, fmt.Errorf("Invalid token")
	}

	return &Claims{
		sub: claims["sub"].(string),
	}, nil
}
