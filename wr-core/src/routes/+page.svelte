<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  let logicGateContent = "Welcome to your Study Assistant.";
  let status = "Ready";
  let currentCycle = "01/36";

  async function fetchLogicGate() {
    try {
      const response = await invoke("get_logic_gate", { id: "ZERO_G_001" });
      const problem = JSON.parse(response as string);
      logicGateContent = problem.content;
    } catch (e) {
      console.error(e);
    }
  }

  async function exitApp() {
    await invoke("exit_application");
  }

  async function clearData() {
    if (confirm("Are you sure you want to clear your study progress?")) {
        await invoke("clear_user_data");
        alert("Progress cleared.");
    }
  }

  onMount(() => {
    fetchLogicGate();
  });
</script>

<main class="dashboard">
  <nav class="top-nav">
    <div class="logo">W.R. STUDY ASSISTANT</div>
    <div class="nav-links">
        <button on:click={fetchLogicGate}>Next Problem</button>
        <button on:click={clearData}>Settings</button>
        <button class="exit-btn" on:click={exitApp}>Exit</button>
    </div>
  </nav>

  <section class="content-area">
    <aside class="sidebar">
      <div class="stats">
        <h3>Progress</h3>
        <div class="stat-item">Cycle: {currentCycle}</div>
        <div class="stat-item">Status: {status}</div>
      </div>
    </aside>

    <article class="main-content">
      <h1>Active Curriculum</h1>
      <div class="problem-card">
        <p>{logicGateContent}</p>
        <div class="actions">
            <input type="text" placeholder="Type your answer here..." />
            <button class="primary">Submit</button>
        </div>
      </div>
    </article>
  </section>
</main>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    background-color: #f4f7f6;
    color: #333;
    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
  }

  .dashboard {
    display: flex;
    flex-direction: column;
    height: 100vh;
  }

  .top-nav {
    background-color: #2c3e50;
    color: white;
    padding: 10px 20px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
  }

  .logo {
    font-weight: bold;
    font-size: 1.2rem;
  }

  .nav-links button {
    background: transparent;
    border: 1px solid white;
    color: white;
    padding: 5px 15px;
    margin-left: 10px;
    cursor: pointer;
    border-radius: 4px;
    transition: background 0.3s;
  }

  .nav-links button:hover {
    background: rgba(255,255,255,0.1);
  }

  .exit-btn {
    border-color: #e74c3c !important;
    color: #e74c3c !important;
  }

  .exit-btn:hover {
    background: #e74c3c !important;
    color: white !important;
  }

  .content-area {
    display: flex;
    flex-grow: 1;
  }

  .sidebar {
    width: 250px;
    background-color: white;
    padding: 20px;
    border-right: 1px solid #ddd;
  }

  .stat-item {
    margin: 10px 0;
    font-size: 1rem;
  }

  .main-content {
    flex-grow: 1;
    padding: 40px;
  }

  h1 {
    margin-top: 0;
  }

  .problem-card {
    background: white;
    padding: 30px;
    border-radius: 8px;
    box-shadow: 0 4px 6px rgba(0,0,0,0.05);
  }

  .problem-card p {
    font-size: 1.1rem;
    line-height: 1.6;
    margin-bottom: 30px;
  }

  .actions {
    display: flex;
    gap: 10px;
  }

  input {
    flex-grow: 1;
    padding: 10px;
    border: 1px solid #ddd;
    border-radius: 4px;
  }

  button.primary {
    background-color: #3498db;
    color: white;
    border: none;
    padding: 10px 20px;
    border-radius: 4px;
    cursor: pointer;
  }

  button.primary:hover {
    background-color: #2980b9;
  }
</style>
