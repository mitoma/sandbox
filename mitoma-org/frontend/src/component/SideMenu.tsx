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
  Collapse,
} from "@mui/material";
import { Link } from "react-router-dom";
import AccountCircleIcon from "@mui/icons-material/AccountCircle";
import HandymanIcon from "@mui/icons-material/Handyman";
import BookIcon from "@mui/icons-material/Book";
import GiteIcon from "@mui/icons-material/Gite";
import ArrowRightIcon from '@mui/icons-material/ArrowRight';
import React, { useState } from "react";

const drawerWidth = 240;

function SideMenu() {
  const [showBlogList, setShowBlogList] = useState(false);

  const toggleShowBlogList = () => setShowBlogList(!showBlogList);
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
            <ListItem key="Blog" disablePadding onClick={toggleShowBlogList}>
              <ListItemButton>
                <ListItemIcon>
                  <ArrowRightIcon/>
                </ListItemIcon>
                <ListItemText primary="Blog" />
              </ListItemButton>
            </ListItem>
            <Collapse in={showBlogList} timeout="auto" unmountOnExit>
              <List component="div" disablePadding>
                <Link to="/blog/create_blog">
                  <ListItemButton sx={{ pl: 4 }}>
                    <ListItemIcon>
                      <BookIcon />
                    </ListItemIcon>
                    <ListItemText primary="Starred" />
                  </ListItemButton>
                </Link>
              </List>
            </Collapse>
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
