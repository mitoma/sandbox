import { Box, Toolbar, Typography } from "@mui/material";

type ContentSpacerProps = {
  showSideMenu: boolean;
};

function ContentSpacer({ showSideMenu }: ContentSpacerProps) {
  return (
    <Toolbar>
      <img src="/logo.png" width="50" alt="profile icon" />
      <Box sx={{ padding: 2 }}>
        <Typography variant="h6">mitoma.org</Typography>
      </Box>
    </Toolbar>
  );
}

export default ContentSpacer;
