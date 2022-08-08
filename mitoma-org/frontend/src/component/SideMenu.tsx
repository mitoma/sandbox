import {
  Drawer,
  Toolbar,
  Box,
  List,
  ListItem,
  ListItemButton,
  ListItemIcon,
  ListItemText,
  Divider,
  Typography,
  IconButton,
  Fab,
} from "@mui/material";
import { Link } from "react-router-dom";
import AccountCircleIcon from "@mui/icons-material/AccountCircle";
import HandymanIcon from "@mui/icons-material/Handyman";
import BookIcon from "@mui/icons-material/Book";
import GiteIcon from "@mui/icons-material/Gite";
import React, { useState } from "react";
import KeyboardDoubleArrowLeftIcon from "@mui/icons-material/KeyboardDoubleArrowLeft";
import KeyboardDoubleArrowRightIcon from "@mui/icons-material/KeyboardDoubleArrowRight";
import useLocalStorage from "../hook/useLocalStorage";

const defaultDrawerWidth = 240;
const closedDrawerWidth = 0;

function SideMenu() {
  const [showSideMenu, setShowSideMenu] = useLocalStorage("showSideMenu", true);

  if (!showSideMenu) {
    return (
      <React.Fragment>
        <Fab
          size="small"
          sx={{
            position: "absolute",
            top: 16,
            left: 16,
            boxShadow: 0,
          }}
        >
          <IconButton onClick={() => setShowSideMenu(true)}>
            <KeyboardDoubleArrowRightIcon />
          </IconButton>
        </Fab>
      </React.Fragment>
    );
  }
  return (
    <React.Fragment>
      <Fab
        size="small"
        sx={{
          position: "absolute",
          top: 16,
          left: defaultDrawerWidth - 16,
          boxShadow: 0,
          zIndex: (theme) => theme.zIndex.drawer + 1,
        }}
      >
        <IconButton onClick={() => setShowSideMenu(false)}>
          <KeyboardDoubleArrowLeftIcon />
        </IconButton>
      </Fab>
      <Drawer
        variant="permanent"
        anchor="left"
        sx={{
          width: showSideMenu ? defaultDrawerWidth : closedDrawerWidth,
          flexShrink: 0,
          [`& .MuiDrawer-paper`]: {
            width: showSideMenu ? defaultDrawerWidth : closedDrawerWidth,
            boxSizing: "border-box",
          },
        }}
      >
        <Toolbar>
          <img src="/logo.png" width="50" alt="profile icon" />
          <Box sx={{ padding: 2 }}>
            <Typography variant="h6">mitoma.org</Typography>
          </Box>
        </Toolbar>
        <Divider />
        <Box sx={{ overflow: "auto" }}>
          <List>
            <Link to="/" style={{ textDecoration: "none" }}>
              <ListItem key="Home" disablePadding>
                <ListItemButton>
                  <ListItemIcon>
                    <GiteIcon />
                  </ListItemIcon>
                  <ListItemText primary="Home" />
                </ListItemButton>
              </ListItem>
            </Link>
            <Link to="/blog" style={{ textDecoration: "none" }}>
              <ListItem key="Blog" disablePadding>
                <ListItemButton>
                  <ListItemIcon>
                    <BookIcon />
                  </ListItemIcon>
                  <ListItemText primary="Blog" />
                </ListItemButton>
              </ListItem>
            </Link>
            <Link to="/tool" style={{ textDecoration: "none" }}>
              <ListItem key="Tool" disablePadding>
                <ListItemButton>
                  <ListItemIcon>
                    <HandymanIcon />
                  </ListItemIcon>
                  <ListItemText primary="Tool" />
                </ListItemButton>
              </ListItem>
            </Link>
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
    </React.Fragment>
  );
}

export default SideMenu;
