package main

import (
	"encoding/json"
	"fmt"
	"net/http"
)

type Message struct {
	Message string `json:"message"`
}

func main() {
	http.HandleFunc("/json", func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		data := Message{Message: "Hello, JSON!"}
		json.NewEncoder(w).Encode(data)
	})

	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "text/html")
		fmt.Fprintf(w, "<html><body><h1>Hello, HTML!</h1></body></html>")
	})

	port := 8002
	fmt.Printf("Server started at port %d\n", port)
	http.ListenAndServe(fmt.Sprintf(":%d", port), nil)
}

