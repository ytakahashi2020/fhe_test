import React, { useState } from "react";

function App() {
  // 状態管理
  const [num1, setNum1] = useState("");
  const [num2, setNum2] = useState("");
  const [result, setResult] = useState(null);

  // 送信処理
  const handleSubmit = async (e) => {
    e.preventDefault();

    try {
      // サーバーへリクエストを送信
      const response = await fetch("http://localhost:8000/add", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ num1: Number(num1), num2: Number(num2) }), // 数値として送信
      });

      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }

      const data = await response.json();
      setResult(data.result); // 結果を状態に保存
    } catch (error) {
      console.error("Error:", error);
    }
  };

  return (
    <div className="App">
      <h1>暗号化足し算アプリ</h1>
      <form onSubmit={handleSubmit}>
        <div>
          <input
            type="number"
            value={num1}
            onChange={(e) => setNum1(e.target.value)}
            placeholder="1つ目の数字"
          />
        </div>
        <div>
          <input
            type="number"
            value={num2}
            onChange={(e) => setNum2(e.target.value)}
            placeholder="2つ目の数字"
          />
        </div>
        <button type="submit">送信</button>
      </form>
      {result !== null && <h2>結果: {result}</h2>} {/* 結果を表示 */}
    </div>
  );
}

export default App;
