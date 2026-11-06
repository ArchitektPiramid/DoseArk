/** App release and frontend versions come directly from their manifests. */
import { appName, version as releaseVersion } from "../../../../../doseark.version.json";
import { version as webVersion } from "../../../package.json";

export const APP_NAME = appName;
export const APP_RELEASE_VERSION = releaseVersion;
export const FRONTEND_VERSION = webVersion;
export const FRONTEND_VERSION_LABEL = `v${FRONTEND_VERSION}`;
