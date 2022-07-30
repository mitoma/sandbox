import {
  Grid,
  Link,
  List,
  ListItem,
  ListItemIcon,
  ListItemText,
  Typography,
} from "@mui/material";
import React from "react";
import TwitterIcon from "@mui/icons-material/Twitter";
import GitHubIcon from "@mui/icons-material/GitHub";

function AboutMe() {
  return (
    <React.Fragment>
      <Typography paragraph variant="h4">
        自己紹介
      </Typography>
      <Grid container spacing={2}>
        <Grid item>
          <img src="/logo.png" />
        </Grid>
        <Grid item>
          <Typography paragraph>
            天下一品歴30余年の大ベテランですぞ。
          </Typography>
          <List>
            <ListItem>
              <ListItemIcon>
                <TwitterIcon />
              </ListItemIcon>
              <ListItemText>
                <Link target="_blank" href="https://twitter.com/mitomasan">
                  mitomasan
                </Link>
              </ListItemText>
            </ListItem>
            <ListItem>
              <ListItemIcon>
                <GitHubIcon />
              </ListItemIcon>
              <ListItemText>
                <Link target="_blank" href="https://github.com/mitoma">
                  mitoma
                </Link>
              </ListItemText>
            </ListItem>
          </List>
        </Grid>
      </Grid>
      <Typography paragraph>
        このサイトはメモ置き場とか適当なツールを置くサンドボックスにしていく予定です。
      </Typography>
    </React.Fragment>
  );
}

export default AboutMe;
