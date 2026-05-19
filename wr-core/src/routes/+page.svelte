<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  let logicGateContent = "Loading curriculum...";
  let problemId = "ZERO_G_001";
  let status = "Idle";
  let currentCycle = "01/36";
  let userAnswer = "";
  let feedbackMessage = "";
  let isLoading = false;

  async function fetchProblem(id: string) {
    isLoading = true;
    feedbackMessage = "";
    try {
      if (typeof invoke === 'undefined') {
          throw new Error("Tauri invoke is not available. Ensure you are running within the Tauri container.");
      }
      const response = await invoke("get_logic_gate", { id });
      const problem = JSON.parse(response as string);
      logicGateContent = problem.content;
      problemId = id;
      status = "Active";
    } catch (e) {
      feedbackMessage = `Error loading problem: ${e}`;
      status = "Error";
    } finally {
      isLoading = false;
    }
  }

  async function submitAnswer() {
    if (!userAnswer.trim()) {
        feedbackMessage = "Please enter an answer.";
        return;
    }
    feedbackMessage = "Answer recorded. Progress synced.";
    userAnswer = "";
    status = "Reviewing";
  }

  async function nextProblem() {
      const nextId = problemId === "ZERO_G_001" ? "ZERO_G_002" : "ZERO_G_003";
      await fetchProblem(nextId);
  }

  async function exitApp() {
    try {
        await invoke("exit_application");
    } catch (e) {
        console.error("Failed to exit:", e);
    }
  }

  async function clearData() {
    if (confirm("Are you sure you want to clear your study progress logs? This action cannot be undone.")) {
        try {
            await invoke("clear_user_data");
            alert("Progress logs cleared.");
        } catch (e) {
            alert(`Failed to clear data: ${e}`);
        }
    }
  }

  onMount(() => {
    fetchProblem(problemId);
  });
</script>

<main class="dashboard">
  <nav class="top-nav">
    <div class="logo">W.R. STUDY ASSISTANT</div>
    <div class="nav-links">
        <button on:click={nextProblem} disabled={isLoading}>Skip Problem</button>
        <button on:click={clearData}>Clear Logs</button>
        <button class="exit-btn" on:click={exitApp}>Exit</button>
    </div>
  </nav>

  <section class="content-area">
    <aside class="sidebar">
      <div class="stats">
        <h3>Your Progress</h3>
        <div class="stat-item">
            <span class="label">Current Cycle:</span>
            <span class="value">{currentCycle}</span>
        </div>
        <div class="stat-item">
            <span class="label">System State:</span>
            <span class="value">{status}</span>
        </div>
      </div>
      <div class="guidance">
          <h4>Assistant Tips</h4>
          <p>Derive your solutions carefully. The curriculum adapts to your processing speed.</p>
      </div>
    </aside>

    <article class="main-content">
      <h1>Active Curriculum</h1>
      <div class="problem-card">
        {#if isLoading}
            <p>Decrypting curriculum data...</p>
        {:else}
            <p class="content-text">{logicGateContent}</p>
        {/if}

        {#if feedbackMessage}
            <div class="feedback {feedbackMessage.includes('Error') ? 'error' : 'info'}">
                {feedbackMessage}
            </div>
        {/if}

        <div class="actions">
            <input
                type="text"
                bind:value={userAnswer}
                placeholder="Type your derivation or solution here..."
                on:keydown={(e) => e.key === 'Enter' && submitAnswer()}
            />
            <button class="primary" on:click={submitAnswer} disabled={isLoading}>Submit</button>
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
    letter-spacing: 1px;
  }

  .nav-links button {
    background: transparent;
    border: 1px solid rgba(255,255,255,0.5);
    color: white;
    padding: 6px 16px;
    margin-left: 10px;
    cursor: pointer;
    border-radius: 4px;
    font-size: 0.9rem;
    transition: all 0.2s;
  }

  .nav-links button:hover:not(:disabled) {
    background: rgba(255,255,255,0.1);
    border-color: white;
  }

  .nav-links button:disabled {
      opacity: 0.5;
      cursor: not-allowed;
  }

  .exit-btn {
    border-color: #e74c3c !important;
    color: #e74c3c !important;
  }

  .exit-btn:hover:not(:disabled) {
    background: #e74c3c !important;
    color: white !important;
  }

  .content-area {
    display: flex;
    flex-grow: 1;
    overflow: hidden;
  }

  .sidebar {
    width: 260px;
    background-color: white;
    padding: 25px;
    border-right: 1px solid #ddd;
    display: flex;
    flex-direction: column;
    gap: 30px;
  }

  .stats h3, .guidance h4 {
      margin-top: 0;
      color: #2c3e50;
      border-bottom: 2px solid #f4f7f6;
      padding-bottom: 8px;
  }

  .stat-item {
    margin: 15px 0;
    display: flex;
    justify-content: space-between;
    font-size: 0.95rem;
  }

  .stat-item .label {
      color: #7f8c8d;
  }

  .stat-item .value {
      font-weight: 600;
      color: #2c3e50;
  }

  .guidance p {
      font-size: 0.9rem;
      color: #7f8c8d;
      line-height: 1.5;
      font-style: italic;
  }

  .main-content {
    flex-grow: 1;
    padding: 40px;
    overflow-y: auto;
    background: linear-gradient(to bottom right, #f4f7f6, #ffffff);
  }

  h1 {
    margin-top: 0;
    color: #2c3e50;
    font-size: 1.8rem;
    margin-bottom: 30px;
  }

  .problem-card {
    background: white;
    padding: 40px;
    border-radius: 12px;
    box-shadow: 0 10px 25px rgba(0,0,0,0.05);
    max-width: 800px;
  }

  .content-text {
    font-size: 1.15rem;
    line-height: 1.7;
    margin-bottom: 30px;
    color: #2c3e50;
    min-height: 100px;
  }

  .feedback {
      margin-bottom: 20px;
      padding: 10px;
      border-radius: 4px;
      font-size: 0.9rem;
      min-height: 20px;
  }

  .feedback.info {
      color: #27ae60;
  }

  .feedback.error {
      color: #c0392b;
      background: #fdf2f2;
  }

  .actions {
    display: flex;
    gap: 15px;
    border-top: 1px solid #f4f7f6;
    padding-top: 30px;
  }

  input {
    flex-grow: 1;
    padding: 12px 16px;
    border: 2px solid #ecf0f1;
    border-radius: 6px;
    font-size: 1rem;
    transition: border-color 0.2s;
  }

  input:focus {
      outline: none;
      border-color: #3498db;
  }

  button.primary {
    background-color: #3498db;
    color: white;
    border: none;
    padding: 12px 24px;
    border-radius: 6px;
    cursor: pointer;
    font-weight: 600;
    transition: background 0.2s;
  }

  button.primary:hover:not(:disabled) {
    background-color: #2980b9;
  }

  button.primary:disabled {
      opacity: 0.7;
      cursor: not-allowed;
  }
</style>
