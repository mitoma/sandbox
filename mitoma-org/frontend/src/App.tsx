import { Box, CssBaseline, Toolbar } from "@mui/material";
import React from "react";
import AboutMe from "./page/AboutMe";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import SideMenu from "./component/SideMenu";
import Header from "./component/Header";
import Diary from "./page/Diary";

const queryClient = new QueryClient();

function App() {
  return (
    <>
      <BrowserRouter>
        <QueryClientProvider client={queryClient}>
          <Box sx={{ display: "flex" }}>
            <CssBaseline />
            <Header />
            <SideMenu />
            <Box component="main" sx={{ flexGrow: 1, p: 3 }}>
              <Toolbar />
              <Routes>
                <Route path="/" element={<div />} />
                <Route path="/diary" element={<Diary />} />
                <Route path="/aboutMe" element={<AboutMe />} />
              </Routes>
            </Box>
          </Box>
        </QueryClientProvider>
      </BrowserRouter>
    </>
  );
}

export default App;
