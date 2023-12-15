let services = []; // List of available services
let dir = "services"; // Directory where services are located

// Function to list directories in a given path (excluding certain directories)
fn list_directories(path, exclude) {
    // Logic to list directories in `path` and exclude certain ones
    // This can be achieved by reading the file system, which might require external tooling
    // Return an array of directory names
}

// Update the `services` array with available service names
services = list_directories(dir, ["non_service_dir1", "non_service_dir2"]);

// Output the available services for user selection
print(`Available services: ${services}`);
