import { Toolbar } from "@mui/material";

type ContentSpacerProps = {
  showSideMenu: boolean;
};

function ContentSpacer({ showSideMenu }: ContentSpacerProps) {
  if (showSideMenu) {
    return <></>;
  } else {
    return <Toolbar />;
  }
}

export default ContentSpacer;
