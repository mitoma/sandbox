import {
  Card,
  CardActions,
  CardContent,
  CardHeader,
  IconButton,
} from "@mui/material";
import React from "react";
import {
  dateToString,
  dateToJapaneseFormatString,
  intervalString,
} from "./calc";
import ArrowUpwardIcon from "@mui/icons-material/ArrowUpward";
import ArrowDownwardIcon from "@mui/icons-material/ArrowDownward";
import DeleteIcon from "@mui/icons-material/Delete";

type Row = {
  name: string;
  date: string;
  eraDate: string;
  age: string;
};

function newRow(name: string, date: Date, currentDate: Date): Row {
  return {
    name,
    date: dateToString(date),
    eraDate: dateToJapaneseFormatString(date),
    age: intervalString(date, currentDate),
  };
}

function CalcAge() {
  const date: Date = new Date();

  const rows: Row[] = [
    newRow("現在時刻", date, date),
    newRow("Age", new Date(1980, 4, 26), date),
  ];

  return (
    <React.Fragment>
      <h1>時間けいさん君</h1>
      作ってる途中
      {rows.map((row) => (
        <Card sx={{ margin: 2 }}>
          <CardHeader title={row.name} />
          <CardContent>
            <p>{row.date}</p>
            <p>{row.eraDate}</p>
            <p>{row.age}</p>
          </CardContent>
          <CardActions>
            <IconButton>
              <ArrowUpwardIcon />
            </IconButton>
            <IconButton>
              <ArrowDownwardIcon />
            </IconButton>
            <IconButton color="error">
              <DeleteIcon />
            </IconButton>
          </CardActions>
        </Card>
      ))}
    </React.Fragment>
  );
}

export default CalcAge;
