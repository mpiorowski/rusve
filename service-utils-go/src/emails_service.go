package main

import (
	"context"
	"encoding/json"
	"log/slog"

	"cloud.google.com/go/pubsub"
	"github.com/sendgrid/sendgrid-go"
	"github.com/sendgrid/sendgrid-go/helpers/mail"
)

type Email struct {
	Email   string
	Subject string
	Message string
}

func subscribe_to_emails() error {
	if ENV == "development" {
		return nil
	}

	ctx := context.Background()
	client, err := pubsub.NewClient(ctx, "rusve-384620")
	if err != nil {
		slog.Error("subscribe_to_emails", "pubsub.NewClient", err)
		return err
	}

	go pull_messages(ctx, client)

	return nil
}

func pull_messages(ctx context.Context, client *pubsub.Client) {
	slog.Info("Email service started")

	sub := client.Subscription("email-sub-go")
	defer client.Close()

	err := sub.Receive(ctx, func(_ context.Context, msg *pubsub.Message) {
		var email Email
		err := json.Unmarshal(msg.Data, &email)
		if err != nil {
			slog.Error("pull_messages", "json.Unmarshal", err)
			msg.Nack()
			return
		}

		from := mail.NewEmail("Rusve - go", "email@rusve.app")
		to := mail.NewEmail(email.Email, email.Email)
		subject := email.Subject
		body := email.Message

		message := mail.NewSingleEmail(from, subject, to, "", body)
		client := sendgrid.NewSendClient(SENDGRID_API_KEY)
		response, err := client.Send(message)
		if err != nil {
            slog.Error("pull_messages", "client.Send", err)
			msg.Nack()
			return
		}
		slog.Info("pull_messages", "response.StatusCode", response.StatusCode)
		msg.Ack()
	})
	if err != nil {
        slog.Error("pull_messages", "sub.Receive", err)
		return
	}
	slog.Info("Email service stopped")
}
