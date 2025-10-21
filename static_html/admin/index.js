document.addEventListener("DOMContentLoaded", () => {
    const loginBtn = document.getElementById("login");

    loginBtn.addEventListener("click", () => {
        const username = document.getElementById("username").value.trim();
        const password = document.getElementById("password").value.trim();

        if (!username || !password) {
            alert("Please enter both username and password.");
            return;
        }

        fetch("/api/v1/auth/login", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                username,
                password
            })
        })
        .then(response => response.json())
        .then(data => {
            if (data.Success && data.Success.token) {
                localStorage.setItem("token", data.Success.token);
                window.location.href = "admin.html";
            } else {
                alert("Invalid username or password");
            }
        })
        .catch(error => {
            console.error("Login failed:", error);
            alert("An error occurred during login.");
        });
    });
});