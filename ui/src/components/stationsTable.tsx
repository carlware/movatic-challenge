import TableContainer from "@mui/material/TableContainer";
import Paper from "@mui/material/Paper";
import Table from "@mui/material/Table";
import TableHead from "@mui/material/TableHead";
import TableRow from "@mui/material/TableRow";
import TableCell from "@mui/material/TableCell";
import TableBody from "@mui/material/TableBody";
import type { StationStatus } from "../api/stations/status";
import * as React from "react";

interface Props {
	stations: StationStatus[];
	handleOpen(station: StationStatus): void;
}

export const StationsTable = ({ stations, handleOpen }: Props) => {
	return (
		<TableContainer component={Paper}>
			<Table sx={{ minWidth: 650 }} aria-label="simple table">
				<TableHead>
					<TableRow>
						<TableCell>Station ID</TableCell>
						<TableCell align="right">Bikes Available</TableCell>
						<TableCell align="right">Docs Available</TableCell>
						<TableCell align="right">Electric</TableCell>
						<TableCell align="right">Classic</TableCell>
						<TableCell align="right">Smart</TableCell>
					</TableRow>
				</TableHead>
				<TableBody>
					{stations.map((station: StationStatus) => (
						<TableRow
							key={station.station_id}
							onClick={() => {
								handleOpen(station);
							}}
						>
							<TableCell component="th" scope="row">
								{station.station_id}
							</TableCell>
							<TableCell align="right">{station.num_bikes_available}</TableCell>
							<TableCell align="right">{station.num_docks_available}</TableCell>
							<TableCell align="right">
								{station.num_bikes_available_types.electric}
							</TableCell>
							<TableCell align="right">
								{station.num_bikes_available_types.classic}
							</TableCell>
							<TableCell align="right">
								{station.num_bikes_available_types.smart}
							</TableCell>
						</TableRow>
					))}
				</TableBody>
			</Table>
		</TableContainer>
	);
};
