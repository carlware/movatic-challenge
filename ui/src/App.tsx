import React from "react";
import "./App.css";
import { Typography } from "@mui/material";
import { StationsView } from "./containers/stationsView";

function App() {
	return (
		<div className="App">
			<Typography variant="h1" align="center" gutterBottom>
				Movatic
			</Typography>
			<StationsView />
		</div>
	);
}

export default App;
