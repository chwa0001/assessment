import {
  BrowserRouter
} from "react-router-dom";

import Routes from "./routes";
// import Home from "./components/home";
import { AppProvider } from "./context/AppContext"
import './main.css'

export default function App() {
  return (
    <BrowserRouter>
    <AppProvider>
      <Routes/>
    </AppProvider>
    </BrowserRouter>
  )
}

