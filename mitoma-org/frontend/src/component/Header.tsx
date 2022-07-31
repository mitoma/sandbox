import { AppBar, Box, Toolbar, Typography } from "@mui/material";
import React from "react";
import HomeIcon from "@mui/icons-material/Home";

function Header() {
  return (
    <React.Fragment>
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
    </React.Fragment>
  );
}

export default Header;
