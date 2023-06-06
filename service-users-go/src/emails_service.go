package main

import (
	"context"
	"encoding/json"
	"log"

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
		log.Printf("NewClient: %v", err)
		return err
	}
	defer client.Close()

	sub := client.Subscription("email-sub-go")

	go pull_messages(ctx, sub)

	return nil
}

func pull_messages(ctx context.Context, sub *pubsub.Subscription) {
	err := sub.Receive(ctx, func(_ context.Context, msg *pubsub.Message) {
		var email Email
		err := json.Unmarshal(msg.Data, &email)
		if err != nil {
			log.Printf("Unmarshal: %v", err)
			msg.Nack()
			return
		}

		from := mail.NewEmail("Rusve", "Rusve - go")
		to := mail.NewEmail(email.Email, email.Email)
		subject := email.Subject
		body := email.Message

		message := mail.NewSingleEmail(from, subject, to, "", body)
		client := sendgrid.NewSendClient(SENDGRID_API_KEY)
		response, err := client.Send(message)
		if err != nil {
			log.Printf("client.Send: %v", err)
			msg.Nack()
			return
		}
		log.Printf("Email sent: %v", response.StatusCode)
		msg.Ack()
	})
	if err != nil {
		log.Printf("Error receiving message: %v", err)
	}
}
