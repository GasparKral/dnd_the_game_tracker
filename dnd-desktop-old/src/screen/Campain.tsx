// pages/Campain.tsx
import { useState, useEffect } from "react";
import { TransformWrapper, TransformComponent } from "react-zoom-pan-pinch";
import { invoke } from "@tauri-apps/api/core";
import SiteInfoCard, { SiteInfoCardProps } from "@component/SiteInfoCard";

interface MapInfo {
	path: string,
	clickableZones: ClickableZone[]
}

interface ClickableZone {
	shape: "rect" | "circle" | "poly",
	coords: number[],
	alt: string,
	displayInfo: SiteInfoCardProps
}

const Campain = () => {
	const [mapInfo, setMapInfo] = useState<MapInfo | null>(null);
	const [imageDimensions, setImageDimensions] = useState({ width: 0, height: 0 });

	// Estado para la tarjeta de información
	const [selectedSite, setSelectedSite] = useState<SiteInfoCardProps['siteInfo'] | null>(null);
	const [isCardOpen, setIsCardOpen] = useState(false);
	const [clickPosition, setClickPosition] = useState<{ x: number; y: number } | undefined>();

	useEffect(() => {
		getBG().then(setMapInfo);
	}, []);

	// Cargar imagen para obtener dimensiones
	useEffect(() => {
		if (mapInfo?.path) {
			const img = new Image();
			img.src = mapInfo.path;
			img.onload = () => {
				setImageDimensions({
					width: img.width,
					height: img.height
				});
			};
		}
	}, [mapInfo?.path]);

	const handleZoneClick = (zone: ClickableZone, e: React.MouseEvent) => {
		e.stopPropagation();

		// Guardar la posición del click para mostrar la tarjeta cerca
		setClickPosition({ x: e.clientX, y: e.clientY });

		// Mostrar la información del sitio
		setSelectedSite(zone.displayInfo.siteInfo);
		setIsCardOpen(true);

		console.log("Sitio seleccionado:", zone.alt, zone.displayInfo);
	};

	const handleCloseCard = () => {
		setIsCardOpen(false);
		setSelectedSite(null);
		setClickPosition(undefined);
	};

	if (!mapInfo) return (
		<div className="h-screen w-screen flex items-center justify-center">
			<div className="text-center">
				<div className="animate-spin rounded-full h-12 w-12 border-b-2 border-gray-900 mx-auto mb-4"></div>
				<p>Cargando mapa...</p>
			</div>
		</div>
	);

	return (
		<main className="h-screen w-screen overflow-hidden relative">
			<TransformWrapper
				initialScale={1}
				minScale={0.5}
				maxScale={5}
				centerOnInit={true}
				wheel={{ step: 0.1 }}
				doubleClick={{ disabled: true }}
				limitToBounds={false}
			>
				{({ zoomIn, zoomOut, resetTransform }) => (
					<>
						{/* Controles de zoom */}
						<div className="absolute top-4 right-4 z-30 flex gap-2 bg-white rounded-lg shadow-lg p-2">
							<button
								onClick={() => zoomIn()}
								className="p-2 hover:bg-gray-100 rounded transition-colors"
								title="Acercar"
							>
								<svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
								</svg>
							</button>
							<button
								onClick={() => zoomOut()}
								className="p-2 hover:bg-gray-100 rounded transition-colors"
								title="Alejar"
							>
								<svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M18 12H6" />
								</svg>
							</button>
							<button
								onClick={() => resetTransform()}
								className="p-2 hover:bg-gray-100 rounded transition-colors"
								title="Restablecer vista"
							>
								<svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
								</svg>
							</button>
						</div>

						<TransformComponent
							wrapperStyle={{ width: "100%", height: "100%" }}
							contentStyle={{ position: "relative" }}
						>
							<div style={{ position: "relative", display: "inline-block" }}>
								{/* Imagen de fondo */}
								<img
									src={mapInfo.path}
									alt="Mapa navegable"
									draggable={false}
									style={{
										maxWidth: "none",
										display: "block",
										userSelect: "none",
										pointerEvents: "none" // Permite que los clicks pasen a las zonas
									}}
								/>

								{/* Zonas cliqueables superpuestas */}
								{imageDimensions.width > 0 && mapInfo.clickableZones.map((zone, index) => (
									<div
										key={index}
										onClick={(e) => handleZoneClick(zone, e)}
										style={{
											position: "absolute",
											...getZoneStyle(zone, imageDimensions),
											cursor: "pointer",
											// Opcional: hover effect
											transition: "background-color 0.2s",
										}}
										title={zone.alt}
										className="hover:bg-yellow-200 hover:bg-opacity-30 border-2 border-transparent hover:border-yellow-400 rounded"
									/>
								))}
							</div>
						</TransformComponent>
					</>
				)}
			</TransformWrapper>

			{/* Tarjeta de información del sitio */}
			{selectedSite && (
				<SiteInfoCard
					isOpen={isCardOpen}
					onClose={handleCloseCard}
					siteInfo={selectedSite}
					position={clickPosition}
				/>
			)}
		</main>
	);
};

// Helper para convertir coordenadas a estilos CSS
const getZoneStyle = (zone: ClickableZone, imgSize: { width: number, height: number }) => {
	switch (zone.shape) {
		case "rect":
			return {
				left: `${(zone.coords[0] / imgSize.width) * 100}%`,
				top: `${(zone.coords[1] / imgSize.height) * 100}%`,
				width: `${((zone.coords[2] - zone.coords[0]) / imgSize.width) * 100}%`,
				height: `${((zone.coords[3] - zone.coords[1]) / imgSize.height) * 100}%`,
			};
		case "circle":
			return {
				left: `${(zone.coords[0] / imgSize.width) * 100}%`,
				top: `${(zone.coords[1] / imgSize.height) * 100}%`,
				width: `${(zone.coords[2] * 2 / imgSize.width) * 100}%`,
				height: `${(zone.coords[2] * 2 / imgSize.height) * 100}%`,
				borderRadius: "50%",
				transform: "translate(-50%, -50%)",
			};
		case "poly":
			// Para polígonos necesitaríamos una implementación más compleja
			// Por ahora, creamos un bounding box aproximado
			const xs = zone.coords.filter((_, i) => i % 2 === 0);
			const ys = zone.coords.filter((_, i) => i % 2 === 1);
			const minX = Math.min(...xs);
			const maxX = Math.max(...xs);
			const minY = Math.min(...ys);
			const maxY = Math.max(...ys);

			return {
				left: `${(minX / imgSize.width) * 100}%`,
				top: `${(minY / imgSize.height) * 100}%`,
				width: `${((maxX - minX) / imgSize.width) * 100}%`,
				height: `${((maxY - minY) / imgSize.height) * 100}%`,
			};
		default:
			return {};
	}
};

export default Campain;

async function getBG(): Promise<MapInfo> {
	return await invoke("getMap");
}
