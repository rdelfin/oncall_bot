/**
 * @prettier
 */

import React from "react";
import AppHeader from "./Header";
import Home from "./Home";
import Users from "./Users";
import Oncalls from "./Oncalls";
import { Routes, Route } from "react-router-dom";
import Box from "@mui/material/Box";
import Container from "@mui/material/Container";

export default function App() {
  return (
    <div>
      <AppHeader />
      <Container maxWidth="lg">
        <Routes>
          <Route path="/" element={<h1>Home</h1>} />
          <Route path="/users" element={<h1>Users</h1>} />
          <Route path="/oncalls" element={<h1>Oncalls</h1>} />
        </Routes>
      </Container>
    </div>
  );
}
