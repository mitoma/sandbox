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
} from "@mui/material";
import { Link } from "react-router-dom";
import AccountCircleIcon from "@mui/icons-material/AccountCircle";
import HandymanIcon from "@mui/icons-material/Handyman";
import BookIcon from "@mui/icons-material/Book";
import React from "react";

const drawerWidth = 240;

function SideMenu() {
  return (
    <React.Fragment>
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
            <Link to="/" style={{ textDecoration: "none" }}>
              <ListItem key="Home" disablePadding>
                <ListItemButton>
                  <ListItemIcon>
                    <AccountCircleIcon />
                  </ListItemIcon>
                  <ListItemText primary="Home" />
                </ListItemButton>
              </ListItem>
            </Link>
            <Link to="/diary/2022-08-01" style={{ textDecoration: "none" }}>
              <ListItem key="Blog" disablePadding>
                <ListItemButton>
                  <ListItemIcon>
                    <BookIcon />
                  </ListItemIcon>
                  <ListItemText primary="Blog" />
                </ListItemButton>
              </ListItem>
            </Link>
            <Link to="/diary/2022-08-01" style={{ textDecoration: "none" }}>
              <ListItem key="Blog" sx={{ pl: 4 }}>
                <ListItemButton>
                  <ListItemIcon>
                    <BookIcon />
                  </ListItemIcon>
                  <ListItemText primary="Blog" />
                </ListItemButton>
              </ListItem>
            </Link>
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
    </React.Fragment>
  );
}

export default SideMenu;
