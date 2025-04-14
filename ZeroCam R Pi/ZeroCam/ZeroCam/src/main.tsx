import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import Settings from "./Settings"
import "bootstrap/dist/css/bootstrap.min.css";
import {createBrowserRouter, RouterProvider} from "react-router-dom"

const router = createBrowserRouter([
  { path:"/", element: <App/>},
  { path:"/settings", element: <Settings/>}
]);

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <RouterProvider router={router} />
  </React.StrictMode>,
);
