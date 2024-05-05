export interface SystemInfo {
  Packages: string;
  Progress: string;
  version: string;
  system: {
    manufacturer: string;
    model: string;
    version: string;
    serial: string;
    uuid: string;
    sku: string;
    virtual: boolean;
  };
  bios: {
    vendor: string;
    version: string;
    releaseDate: string;
    revision?: string;
    serial?: string;
  };
  baseboard: {
    manufacturer: string;
    model: string;
    version: string;
    serial?: string;
    assetTag?: string;
    memMax?: number | null;
    memSlots?: number | null;
  };
  chassis: {
    manufacturer: string;
    model?: string;
    type: string;
    version?: string;
    serial?: string;
    assetTag?: string;
    sku?: string;
  };
  os: {
    platform: string;
    distro: string;
    release: string;
    codename: string;
    kernel: string;
    arch: string;
    hostname: string;
    fqdn: string;
    codepage: string;
    logofile: string;
    serial: string;
    build?: string;
    servicepack?: string;
    uefi: boolean;
  };
  uuid: {
    os: string;
    hardware?: string;
    macs: string[];
  };
  versions: {
    kernel: string;
    openssl: string;
    systemOpenssl: string;
    systemOpensslLib: string;
    node: string;
    v8: string;
    npm: string;
    yarn: string;
    pm2?: string;
    gulp?: string;
    grunt?: string;
    git: string;
    tsc: string;
    mysql?: string;
    redis?: string;
    mongodb?: string;
    apache: string;
    nginx?: string;
    php?: string;
    docker: string;
    postfix?: string;
    postgresql?: string;
    perl: string;
    python: string;
    python3: string;
    pip: string;
    pip3: string;
    java: string;
    gcc: string;
    virtualbox?: string;
    bash: string;
    zsh: string;
    fish?: string;
    powershell?: string;
    dotnet?: string;
  };
  cpu: {
    manufacturer: string;
    brand: string;
    vendor: string;
    family: string;
    model: string;
    stepping: string;
    revision?: string;
    voltage?: string;
    speed: number;
    speedMin: number;
    speedMax: number;
    governor: string;
    cores: number;
    physicalCores: number;
    performanceCores: number;
    efficiencyCores: number;
    processors: number;
    socket?: string;
    flags: string;
    virtualization: boolean;
    cache: {
      l1d: number;
      l1i: number;
      l2: number;
      l3: number;
    };
  };
  graphics: {
    controllers: GraphicsController[];
    displays: Display[];
  };
  net: NetworkInterface[];
  memLayout: MemoryLayout[];
  diskLayout: DiskLayout[];
  time: {
    current: number;
    uptime: number;
    timezone: string;
    timezoneName: string;
  };
}

interface GraphicsController {
  vendor: string;
  subVendor?: string;
  model: string;
  bus: string;
  busAddress: string;
  vram: number;
  vramDynamic: boolean;
  pciID?: string;
  driverVersion?: string;
  subDeviceId?: string;
  name?: string;
  pciBus?: string;
  memoryTotal?: number;
  memoryUsed?: number;
  memoryFree?: number;
  utilizationGpu?: number;
  utilizationMemory?: number;
  temperatureGpu?: number;
  powerDraw?: number;
  clockCore?: number;
  clockMemory?: number;
}

interface Display {
  vendor?: string;
  model?: string;
  deviceName?: string;
  main: boolean;
  builtin: boolean;
  connection: string;
  sizeX?: number | null;
  sizeY?: number | null;
  pixelDepth?: number | null;
  resolutionX?: number | null;
  resolutionY?: number | null;
  currentResX: number;
  currentResY: number;
  positionX: number;
  positionY: number;
  currentRefreshRate: number;
}

interface NetworkInterface {
  iface: string;
  ifaceName: string;
  default: boolean;
  ip4: string;
  ip4subnet: string;
  ip6: string;
  ip6subnet: string;
  mac: string;
  internal: boolean;
  virtual: boolean;
  operstate: string;
  type: string;
  duplex?: string;
  mtu: number;
  speed?: number | null;
  dhcp: boolean;
  dnsSuffix: string;
  ieee8021xAuth: string;
  ieee8021xState: string;
  carrierChanges: number;
}

interface MemoryLayout {
  size: number;
  bank?: string;
  type?: string;
  ecc?: boolean | null;
  clockSpeed?: number;
  formFactor?: string;
  partNum?: string;
  serialNum?: string;
  voltageConfigured?: number | null;
  voltageMin?: number | null;
  voltageMax?: number | null;
}

interface DiskLayout {
  device: string;
  type: string;
  name: string;
  vendor: string;
  size: number;
  bytesPerSector?: number | null;
  totalCylinders?: number | null;
  totalHeads?: number | null;
  totalSectors?: number | null;
  totalTracks?: number | null;
  tracksPerCylinder?: number | null;
  sectorsPerTrack?: number | null;
  firmwareRevision: string;
  serialNum: string;
  interfaceType: string;
  smartStatus?: string;
  temperature?: number | null;
}
[];
