import { getStationsStatus, type StationStatus } from "../api/stations/status";
import { useQuery } from "react-query";
import { StationsTable } from "../components/stationsTable";
import { Typography } from "@mui/material";
import { useState } from "react";
import { StationModal } from "../components/stationModal";

export const StationsView = () => {
	const [open, setOpen] = useState(false);
	const [station, setStation] = useState<StationStatus>();

	const handleOpen = (station: StationStatus) => {
		setOpen(true);
		setStation(station);
	};
	const handleClose = () => {
		setOpen(false);
		setStation(undefined);
	};

	const {
		data: stations,
		isSuccess,
		isError,
	} = useQuery<StationStatus[]>("stations_status", getStationsStatus, {
		staleTime: 1000,
	});

	if (isSuccess) {
		return (
			<>
				<StationsTable stations={stations} handleOpen={handleOpen} />
				<StationModal
					isOpen={open && station !== undefined}
					station={station}
					handleClose={handleClose}
				/>
			</>
		);
	}

	if (isError) {
		return (
			<Typography variant="h3" gutterBottom>
				{" "}
				Error: Try refreshing the page
			</Typography>
		);
	}

	return (
		<Typography variant="h3" gutterBottom>
			{" "}
			Loading ...
		</Typography>
	);
};
