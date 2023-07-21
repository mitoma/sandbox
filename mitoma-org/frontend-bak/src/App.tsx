import { CssBaseline, ThemeProvider } from "@mui/material";
import React from "react";
import AboutMe from "./page/AboutMe";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import Blog from "./page/Blog";
import Home from "./page/Home";
import BlogList from "./page/BlogList";

import { createTheme } from "@mui/material/styles";
import BasicLayout from "./page/BasicLayout";
import NotFound from "./page/NotFound";
import ToolList from "./page/ToolList";
import CalcAge from "./page/tool/CalcAge";

const queryClient = new QueryClient();

const theme = createTheme({
  palette: {
    primary: {
      main: "#7777dd",
    },
    secondary: {
      main: "#55bb55",
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
            <Routes>
              <Route element={<BasicLayout />}>
                <Route path="/" element={<Home />} />
                <Route path="/blog" element={<BlogList />} />
                <Route path="/blog/:blogPath" element={<Blog />} />
                <Route path="/tool" element={<ToolList />} />
                <Route path="/tool/calcAge" element={<CalcAge />} />
                <Route path="/aboutMe" element={<AboutMe />} />
                <Route path="*" element={<NotFound />} />
              </Route>
            </Routes>
          </QueryClientProvider>
        </BrowserRouter>
      </ThemeProvider>
    </>
  );
}

export default App;
