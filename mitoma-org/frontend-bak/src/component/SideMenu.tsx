import {
  Drawer,
  Box,
  List,
  ListItem,
  ListItemButton,
  ListItemIcon,
  ListItemText,
  Divider,
  Toolbar,
} from "@mui/material";
import { Link } from "react-router-dom";
import AccountCircleIcon from "@mui/icons-material/AccountCircle";
import HandymanIcon from "@mui/icons-material/Handyman";
import BookIcon from "@mui/icons-material/Book";
import GiteIcon from "@mui/icons-material/Gite";
import React from "react";

const defaultDrawerWidth = 240;

type SideMenuProps = {
  showSideMenu: boolean;
  setShowSideMenu: (value: boolean) => void;
};

function SideMenu({ showSideMenu, setShowSideMenu }: SideMenuProps) {
  return (
    <React.Fragment>
      <Drawer
        anchor="left"
        variant="temporary"
        open={showSideMenu}
        onClose={() => {
          setShowSideMenu(false);
        }}
        sx={{
          width: defaultDrawerWidth,
          flexShrink: 0,
          [`& .MuiDrawer-paper`]: {
            width: defaultDrawerWidth,
            boxSizing: "border-box",
          },
        }}
      >
        <Toolbar />
        <Box sx={{ overflow: "auto" }}>
          <List>
            <Link
              to="/"
              style={{ textDecoration: "none" }}
              onClick={() => {
                setShowSideMenu(false);
              }}
            >
              <ListItem key="Home" disablePadding>
                <ListItemButton>
                  <ListItemIcon>
                    <GiteIcon />
                  </ListItemIcon>
                  <ListItemText primary="Home" />
                </ListItemButton>
              </ListItem>
            </Link>
            <Link
              to="/blog"
              style={{ textDecoration: "none" }}
              onClick={() => {
                setShowSideMenu(false);
              }}
            >
              <ListItem key="Blog" disablePadding>
                <ListItemButton>
                  <ListItemIcon>
                    <BookIcon />
                  </ListItemIcon>
                  <ListItemText primary="Blog" />
                </ListItemButton>
              </ListItem>
            </Link>
            <Link
              to="/tool"
              style={{ textDecoration: "none" }}
              onClick={() => {
                setShowSideMenu(false);
              }}
            >
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
        <Link
          to="/aboutMe"
          style={{ textDecoration: "none" }}
          onClick={() => {
            setShowSideMenu(false);
          }}
        >
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
