import React, { useState } from 'react';
import axios from 'axios';

function App() {
  const [code, setCode] = useState('');
  const [result, setResult] = useState(null);

  const analyzeCode = async () => {
    try {
      const response = await axios.post('http://localhost:8000/analyze', { code });
      setResult(response.data);
    } catch (error) {
      console.error('Error analyzing code:', error);
    }
  };

  return (
    <div>
      <h1>Time Complexity Analyzer</h1>
      <textarea value={code} onChange={(e) => setCode(e.target.value)} />
      <button onClick={analyzeCode}>Analyze</button>
      {result && <div>Result: {result}</div>}
    </div>
  );
}

export default App;
