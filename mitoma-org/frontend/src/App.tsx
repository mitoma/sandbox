import { Box, CssBaseline } from "@mui/material";
import React from "react";
import AboutMe from "./page/AboutMe";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import SideMenu from "./component/SideMenu";
import Blog from "./page/Blog";
import Home from "./page/Home";

const queryClient = new QueryClient();

function App() {
  return (
    <>
      <BrowserRouter>
        <QueryClientProvider client={queryClient}>
          <Box sx={{ display: "flex" }}>
            <CssBaseline />
            <SideMenu />
            <Box component="main" sx={{ flexGrow: 1, p: 3 }}>
              <Routes>
                <Route path="/" element={<Home />} />
                <Route path="/blog/:blogPath" element={<Blog />} />
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
