# HNG13 Stage 0 - Dynamic Profile Endpoint

## Description
A simple RESTful API built with **Rust (Actix Web)** that returns my profile info and a dynamic cat fact from the Cat Facts API.

## Endpoint
**GET** `/me`  
Example Response:
```json
{
  "status": "success",
  "user": {
    "email": "arabiusman99@gmail.com",
    "name": "Abubakar Abdulazeez Usman",
    "stack": "Rust/Actix Web"
  },
  "timestamp": "2025-10-18T10:59:19.979Z",
  "fact": "A random cat fact..."
}
