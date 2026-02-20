import "./App.css"

import ReactDOM from "react-dom/client";
import App from "./App";
import CampainLayout from "@layout/CampainLayout";
import Campain from "@screen/Campain";

import { BrowserRouter, Routes, Route } from "react-router";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
	<BrowserRouter>
		<Routes>
			<Route path="/" element={<App />} />
			<Route path="/campain_creation" />
			<Route path="/options" />
			<Route element={<CampainLayout />}>
				<Route path="/campain" element={<Campain />} />
				<Route path="" />
			</Route>
		</Routes>
	</BrowserRouter>
);
