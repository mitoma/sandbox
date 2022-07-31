import {
  AppBar,
  Box,
  CssBaseline,
  Divider,
  Drawer,
  List,
  ListItem,
  ListItemButton,
  ListItemIcon,
  ListItemText,
  Toolbar,
  Typography,
} from "@mui/material";
import React, { useState } from "react";
import HomeIcon from "@mui/icons-material/Home";
import AccountCircleIcon from "@mui/icons-material/AccountCircle";
import HandymanIcon from "@mui/icons-material/Handyman";
import BookIcon from "@mui/icons-material/Book";
import AboutMe from "./AboutMe";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { BrowserRouter, Link, Route, Routes } from "react-router-dom";

const drawerWidth = 240;

const queryClient = new QueryClient();

function App() {
  return (
    <>
      <BrowserRouter>
        <QueryClientProvider client={queryClient}>
          <Box sx={{ display: "flex" }}>
            <CssBaseline />
            <AppBar
              position="fixed"
              sx={{ zIndex: (theme) => theme.zIndex.drawer + 1 }}
            >
              <Toolbar>
                <HomeIcon />
                <Box>
                  <Typography variant="h6">mitoma.org</Typography>
                </Box>
              </Toolbar>
            </AppBar>
            <Drawer
              variant="permanent"
              anchor="left"
              sx={{
                width: drawerWidth,
                flexShrink: 0,
                [`& .MuiDrawer-paper`]: {
                  width: drawerWidth,
                  boxSizing: "border-box",
                },
              }}
            >
              <Toolbar />
              <Box sx={{ overflow: "auto" }}>
                <List>
                  <ListItem key="Blog" disablePadding>
                    <ListItemButton>
                      <ListItemIcon>
                        <BookIcon />
                      </ListItemIcon>
                      <ListItemText primary="Blog" />
                    </ListItemButton>
                  </ListItem>
                  <ListItem key="Tool" disablePadding>
                    <ListItemButton>
                      <ListItemIcon>
                        <HandymanIcon />
                      </ListItemIcon>
                      <ListItemText primary="Tool" />
                    </ListItemButton>
                  </ListItem>
                </List>
              </Box>
              <Divider />
              <Link to="/aboutMe" style={{ textDecoration: "none" }}>
                <List>
                  <ListItem key="自己紹介" disablePadding>
                    <ListItemButton>
                      <ListItemIcon>
                        <AccountCircleIcon />
                      </ListItemIcon>
                      <ListItemText primary="自己紹介" />
                    </ListItemButton>
                  </ListItem>
                </List>
              </Link>
            </Drawer>
            <Box component="main" sx={{ flexGrow: 1, p: 3 }}>
              <Toolbar />
              <Routes>
                <Route path="/" element={<div />} />
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
