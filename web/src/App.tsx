import { Navbar } from '@fliqqs/portfolio-scaffold'
import WasmDemo from './components/WasmDemo'
import './App.css'

function App() {

  return (
    <div className="app">
      <Navbar
        title="3D Projection Helper"
        siteName="fliqqs"
        homeUrl="https://fliqqs.github.io"
        links={[]}
      />
      <main className="container">
        <div className="description">
          <p>
            A demo of common elements used in computer graphics, showing world and view space and how it goes to projects to the screen. 
            for networked games. Built with Rust and WebAssembly.
          </p>
        </div>
        <WasmDemo />
      </main>
    </div>
  )
}

export default App
