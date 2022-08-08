import { Box, Container, CssBaseline, ThemeProvider } from "@mui/material";
import React from "react";
import AboutMe from "./page/AboutMe";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import SideMenu from "./component/SideMenu";
import Blog from "./page/Blog";
import Home from "./page/Home";
import BlogList from "./page/BlogList";

import { createTheme } from "@mui/material/styles";
import { purple } from "@mui/material/colors";

const queryClient = new QueryClient();

const theme = createTheme({
  palette: {
    primary: {
      main: purple[500],
    },
    secondary: {
      main: "#f44336",
    },
  },
});

function App() {
  return (
    <>
      <ThemeProvider theme={theme}>
        <CssBaseline enableColorScheme />
        <BrowserRouter>
          <QueryClientProvider client={queryClient}>
            <Box sx={{ display: "flex" }}>
              <SideMenu />
              <Container maxWidth="md" component="main">
                <Routes>
                  <Route path="/" element={<Home />} />
                  <Route path="/blog" element={<BlogList />} />
                  <Route path="/blog/:blogPath" element={<Blog />} />
                  <Route path="/aboutMe" element={<AboutMe />} />
                </Routes>
              </Container>
            </Box>
          </QueryClientProvider>
        </BrowserRouter>
      </ThemeProvider>
    </>
  );
}

export default App;
