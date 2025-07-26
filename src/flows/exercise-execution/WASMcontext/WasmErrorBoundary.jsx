import { Component } from 'react';

export class WasmErrorBoundary extends Component {
  state = { hasError: false, error: null };
  
  static getDerivedStateFromError(error) {
    return { hasError: true, error };
  }

  componentDidCatch(error, info) {
    console.error('WASM Error:', error, info);
  }

  render() {
    if (this.state.hasError) {
      return (
        <div className="wasm-error">
          <h3>WASM Runtime Error</h3>
          <p>{this.state.error.message}</p>
          <button onClick={() => window.location.reload()}>
            Retry
          </button>
        </div>
      );
    }
    return this.props.children; 
  }
}
