import {
	Box,
	Card,
	CardContent,
	CardMedia,
	Modal,
	Typography,
} from "@mui/material";
import { useQuery } from "react-query";
import {
	getStationsInformation,
	type StationInformation,
} from "../api/stations/information";
import type { StationStatus } from "../api/stations/status";
import { getStaticMapURL } from "../utils/map";

const style = {
	position: "absolute",
	top: "50%",
	left: "50%",
	transform: "translate(-50%, -50%)",
	width: "520px",
	bgcolor: "background.paper",
	boxShadow: 24,
	p: 3,
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

	const staticMapURL = getStaticMapURL(
		stationInformation?.lat,
		stationInformation?.lon,
	);

	return (
		<Modal open={isOpen} onClose={handleClose}>
			<Box sx={style}>
				<Card>
					<CardContent>
						<Typography gutterBottom variant="h4" component="div">
							{stationInformation?.name}
						</Typography>
						{staticMapURL && (
							<CardMedia
								sx={{ height: 300, width: 500 }}
								image={staticMapURL}
								title="location"
							/>
						)}
						<Typography
							gutterBottom
							variant="h5"
							component="div"
							sx={{ color: "text.secondary" }}
						>
							Id: {station?.station_id}
						</Typography>
						<Typography
							variant="h6"
							component="div"
							sx={{ color: "text.secondary" }}
						>
							Bikes Available/Docks Available: {station?.num_bikes_available}/
							{station?.num_docks_available}
						</Typography>
						<Typography variant="body1" sx={{ color: "text.secondary" }}>
							Electric: {station?.num_bikes_available_types.electric}
						</Typography>
						<Typography variant="body1" sx={{ color: "text.secondary" }}>
							Classic: {station?.num_bikes_available_types.classic}
						</Typography>
						<Typography variant="body1" sx={{ color: "text.secondary" }}>
							Smart: {station?.num_bikes_available_types.smart}
						</Typography>
					</CardContent>
				</Card>
			</Box>
		</Modal>
	);
};
