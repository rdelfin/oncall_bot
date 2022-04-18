/**
 * @prettier
 */

import React from "react";
import AppHeader from "./Header";
import Home from "./Home";
import Users from "./Users";
import Oncalls from "./Oncalls";
import { Routes, Route } from "react-router-dom";
import Container from "@mui/material/Container";

export default function App() {
  return (
    <div>
      <AppHeader />
      <Container maxWidth="lg">
        <div style={{ padding: "3em" }}>
          <Routes>
            <Route path="/" element={<Home />} />
            <Route path="/users" element={<Users />} />
            <Route path="/oncalls" element={<Oncalls />} />
          </Routes>
        </div>
      </Container>
    </div>
  );
}
