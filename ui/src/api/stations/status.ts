import axios, { type AxiosResponse } from "axios";
const { REACT_APP_API_URL } = process.env;

export interface AvailableBikes {
	electric: number;
	smart: number;
	classic: number;
}

export interface StationStatus {
	is_returning: number;
	is_renting: number;
	is_installed: number;
	num_docks_available: number;
	num_bikes_available: number;
	last_reported: number;
	num_bikes_available_types: AvailableBikes;
	station_id: string;
}

export interface StationStatusResponse {
	last_updated: number;
	data: {
		stations: StationStatus[];
	};
	version: string;
}

export const getStationsStatus = async () => {
	const resp: AxiosResponse<StationStatusResponse> = await axios.get(
		`${REACT_APP_API_URL}/station_status`,
	);
	return resp.data.data.stations;
};
