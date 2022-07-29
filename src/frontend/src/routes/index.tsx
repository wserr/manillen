import { Route, Routes } from "react-router-dom"
import Home from "../pages/Home"
import Lobby from "../pages/Lobby"

export const Router = () =>
    <Routes>
        <Route element={<Layout />}>
        <Route path="/home" element={<Home></Home>} />
        </Route>
    </Routes>

const Layout = () => 
    <h1>THis is a layout</h1>

