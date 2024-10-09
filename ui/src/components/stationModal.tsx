import { Box, Modal, Typography } from "@mui/material";
import { useQuery } from "react-query";
import {
	getStationsInformation,
	type StationInformation,
} from "../api/stations/information";
import {StationStatus} from "../api/stations/status";

const style = {
	position: "absolute",
	top: "50%",
	left: "50%",
	transform: "translate(-50%, -50%)",
	width: "50%",
	bgcolor: "background.paper",
	border: "2px solid #000",
	boxShadow: 24,
	p: 4,
};

interface Props {
	isOpen: boolean;
	station?: StationStatus;
	handleClose(): void;
}

export const StationModal = ({ isOpen, handleClose, station }: Props) => {
	const { data: stations } = useQuery<StationInformation[]>(
		"stations_information",
		getStationsInformation,
		{
			staleTime: 1000,
		},
	);

	const stationInformation = stations?.find(
		(s) => s.station_id === station?.station_id,
	);

	return (
		<Modal open={isOpen} onClose={handleClose}>
			<Box sx={style}>
				<Typography variant="h6" component="h2">
					{station?.station_id}
				</Typography>
				<Typography variant="h5">
					Electric {station?.num_bikes_available_types.electric}
				</Typography>
				<Typography variant="h5">
					Address {stationInformation?.address}
				</Typography>
			</Box>
		</Modal>
	);
};
