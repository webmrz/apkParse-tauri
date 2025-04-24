export interface Permission {
  name: string;
  is_dangerous: boolean;
}

export interface SignatureInfo {
  issuer: string;
  subject: string;
  valid_from: string;
  valid_to: string;
  fingerprint_sha1?: string;
  fingerprint_sha256?: string;
}

export interface FileInfo {
  file_name: string;
  file_size: number;
  file_type: string;
  entry_count?: number;
  md5: string;
  sha1: string;
  sha256: string;
}

export interface ApkInfo {
  package_name: string;
  version_name: string;
  version_code: string;
  min_sdk: string;
  target_sdk: string;
  permissions?: Permission[];
  signature_info?: SignatureInfo;
  file_info?: FileInfo;
  permission_stats?: {
    total: number;
    dangerous: number;
  };
  formatted_version_info?: string;
  formatted_sdk_info?: string;
  icon_base64?: string;
} 