import { useState } from "react";
import { useNavigate } from "react-router-dom";

import { api, resolveDesktopStartupState } from "../api/client";

export function SuperadminPage() {
  const navigate = useNavigate();

  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [fullname, setFullname] = useState("");
  const [error, setError] = useState("");
  const [info, setInfo] = useState("");

  async function createSuperadmin(e: React.FormEvent) {
    e.preventDefault();

    try {
      setError("");
      await api.createSuperadmin({
        username,
        password,
        fullname: fullname.trim() ? fullname.trim() : undefined,
      });

      setInfo("Superadmin user created.");

      const state = await resolveDesktopStartupState();
      if (state.stage === "login") {
        navigate("/login", { replace: true });
      }
    } catch (e) {
      setError(e instanceof Error ? e.message : String(e));
    }
  }

  return (
    <main className="container">
      <h1>Create Superadmin</h1>

      {error && (
        <div
          style={{
            color: "#f87171",
            marginBottom: "1rem",
            padding: "0.5rem 1rem",
            background: "#450a0a",
            borderRadius: "6px",
          }}
        >
          ❌ {error}
        </div>
      )}

      {info && !error && (
        <div
          style={{
            color: "#86efac",
            marginBottom: "1rem",
            padding: "0.5rem 1rem",
            background: "#052e16",
            borderRadius: "6px",
          }}
        >
          ✅ {info}
        </div>
      )}

      <section style={{ maxWidth: "640px", margin: "0 auto", width: "100%" }}>
        <form onSubmit={createSuperadmin}>
          <div className="row" style={{ gap: "0.6rem", flexWrap: "wrap" }}>
            <input
              value={username}
              onChange={(e) => setUsername(e.target.value)}
              placeholder="Username"
              minLength={3}
              required
            />
            <input
              type="password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              placeholder="Password"
              minLength={6}
              required
            />
            <input
              value={fullname}
              onChange={(e) => setFullname(e.target.value)}
              placeholder="Full name (optional)"
            />
            <button type="submit">Create Superadmin</button>
          </div>
        </form>
      </section>
    </main>
  );
}
