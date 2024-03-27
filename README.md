## Rust Tokio Service with MongoDB Example

This example demonstrates a simple Rust service built using the Tokio asynchronous runtime. The service provides an API
connected to a MongoDB database.

The project follows the principles of clean architecture and incorporates best practices in Rust development.

### Features and Roadmap

###### Current Features:

- [X] Asynchronous Design: Leveraging the Tokio asynchronous runtime for efficient and non-blocking I/O operations.
- [X] Graceful Shutdown: Implementing a graceful shutdown mechanism to ensure that the service shuts down cleanly
  without losing data or connections.
- [X] Error Handling: Enhancing error handling mechanisms to gracefully handle errors and failures, ensuring robustness
  and reliability.
- [X] MongoDB Integration: Utilizing MongoDB as the backend database to store and retrieve data.
- [X] Clean Architecture: Following the principles of clean architecture to promote separation of concerns and
  maintainability.
- [X] Logging: Implementing comprehensive logging to capture and analyze service events, errors, and operational
  information.
- [X] Documentation: Improving code documentation and providing comprehensive API documentation for better usability and
  maintainability.
- [X] Testing: Implementing unit tests, integration tests, and possibly property-based testing to ensure the reliability
  and correctness of the service.

###### Roadmap for Future Development:

- [ ] Metrics via Prometheus: Integrating Prometheus for collecting and monitoring service metrics to gain insights into
  performance and resource utilization.
- [ ] Kafka Integration
- [ ] Security Enhancements: Adding authentication and authorization mechanisms to secure the API endpoints and data
  access.
- [ ] Traceability: Enhancing traceability by implementing distributed tracing to monitor and debug requests as they
  traverse through the system.
- [ ] Performance Optimization: Identifying and optimizing performance bottlenecks to improve service scalability and
  responsiveness.

### Usage:

To run the example service via cargo, follow these steps:

1. Install Rust and Cargo if you haven't already.
2. Set up mongo locally:```docker run --name my-mongo-container -d -p 27017:27017 mongo```
3. Navigate to the project directory and run:```cargo run```.
4. Access the Documented API at http://0.0.0.0:8080/swagger-ui/.

To run the example service via docker compose, follow these steps:

1. Install Docker Compose if you haven't already.
2. Build the docker image:```docker compose build```
3. Run the docker compose:```docker compose up```.
4. Access the Documented API at http://0.0.0.0:8080/swagger-ui/.

###### Contributions

Contributions and feedback are welcome! Feel free to open issues or pull requests for bug fixes, feature enhancements,
or suggestions for improvement.
