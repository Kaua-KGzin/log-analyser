use anyhow::Result;
use std::fs::File;
use std::io::Write;

pub fn create_sample_log(path: &str) -> Result<()> {
    let mut file = File::create(path)?;

    let entries = [
        "2024-02-02 10:00:01 INFO  Starting application server on port 8080",
        "2024-02-02 10:00:02 INFO  Database connection established (host=db.prod:5432)",
        "2024-02-02 10:00:10 DEBUG Health-check passed for service auth",
        "2024-02-02 10:00:15 WARNING High memory usage detected: 85% (threshold 80%)",
        "2024-02-02 10:00:30 INFO  User login: user123 from IP 192.168.1.50",
        "2024-02-02 10:00:45 ERROR Failed to connect to payment gateway: timeout after 30s",
        "2024-02-02 10:01:00 ERROR Database query failed: connection lost (retry 1/3)",
        "2024-02-02 10:01:05 ERROR Database query failed: connection lost (retry 2/3)",
        "2024-02-02 10:01:10 WARNING Slow query detected: SELECT * FROM orders took 2.5s",
        "2024-02-02 10:01:20 INFO  User logout: user123",
        "2024-02-02 10:01:30 DEBUG Cache miss for key session:abc123",
        "2024-02-02 10:01:45 ERROR API rate limit exceeded for IP 192.168.1.100",
        "2024-02-02 10:02:00 INFO  Processing batch job: 1000 items queued",
        "2024-02-02 10:02:10 WARNING Disk space low: 10% remaining on /var/data",
        "2024-02-02 10:02:30 ERROR Authentication failed for user: admin from IP 10.0.0.99",
        "2024-02-02 10:02:31 ERROR Authentication failed for user: admin from IP 10.0.0.99",
        "2024-02-02 10:02:45 INFO  Cache cleared successfully",
        "2024-02-02 10:03:00 WARNING CPU usage spike: 95% on worker-3",
        "2024-02-02 10:03:15 ERROR File not found: /var/data/config.yml",
        "2024-02-02 10:03:20 DEBUG Retrying request to upstream (attempt 2)",
        "2024-02-02 10:03:30 INFO  User login: alice from IP 203.0.113.42",
        "2024-02-02 10:03:45 INFO  User login: bob from IP 203.0.113.10",
        "2024-02-02 10:04:00 ERROR SQL error: deadlock detected on table payments",
        "2024-02-02 10:04:15 INFO  Batch job completed: 998/1000 items processed",
        "2024-02-02 10:04:30 WARNING Memory usage spike: 92% — OOM risk",
        "2024-02-02 10:04:45 ERROR Unauthorized access attempt on /admin from IP 192.168.1.100",
        "2024-02-02 10:05:00 INFO  Scheduled maintenance window starting",
        "2024-02-02 10:05:15 DEBUG Thread pool resized to 32 workers",
        "2024-02-02 10:05:30 WARNING Disk space low: 8% remaining on /var/data",
        "2024-02-02 10:05:45 INFO  All services healthy — uptime 99.97%",
    ];

    for entry in &entries {
        writeln!(file, "{}", entry)?;
    }

    Ok(())
}
